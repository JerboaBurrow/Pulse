use axum::extract::State;
use axum::http::{StatusCode, HeaderMap};
use axum::response::IntoResponse;

use axum::
{
    http::Request,
    middleware::Next,
    response::Response,
    body::Bytes,
};

use chrono::Local;
use openssl::hash::MessageDigest;
use openssl::memcmp;
use openssl::pkey::PKey;
use openssl::sign::Signer;
use regex::Regex;

use std::collections::HashMap;

use crate::web::request::discord::{model::Webhook, post::post};

use crate::util::{dump_bytes, read_bytes, strip_control_characters};

#[derive(Clone)]
pub struct GithubConfig
{
    token: String,
    discord: Webhook
}

impl GithubConfig
{
    pub fn new(t: String, w: Webhook) -> GithubConfig
    {
        GithubConfig {token: t, discord: w}
    } 
}

#[derive(Debug)]
enum GithubReleaseActionType
{
    CREATED,
    DELETED,
    EDITED,
    PRERELEASED,
    PUBLISHED,
    RELEASED,
    UNPUBLISHED,
    UNKOWN
}

impl From<&str> for GithubReleaseActionType
{
    fn from(s: &str) -> GithubReleaseActionType
    {
        match s 
        {
            "created" => Self::CREATED,
            "deleted" => Self::DELETED,
            "edited" => Self::EDITED,
            "prereleased" => Self::PRERELEASED,
            "published" => Self::PUBLISHED,
            "released" => Self::RELEASED,
            "unpublished" => Self::UNPUBLISHED,
            _ => Self::UNKOWN
        }
    }
}

impl Into<String> for GithubReleaseActionType
{
    fn into(self: GithubReleaseActionType) -> String 
    {
        match self
        {
            GithubReleaseActionType::CREATED => "created".to_string(),
            GithubReleaseActionType::DELETED => "deleted".to_string(),
            GithubReleaseActionType::EDITED => "edited".to_string(),
            GithubReleaseActionType::PRERELEASED => "prereleased".to_string(),
            GithubReleaseActionType::PUBLISHED => "published".to_string(),
            GithubReleaseActionType::RELEASED => "released".to_string(),
            GithubReleaseActionType::UNPUBLISHED => "unpublished".to_string(),
            _ => "unkown".to_string()
        }
    }
}

/// Middleware to detect, verify, and respond to a github POST request from a 
/// Github webhook
/// 
/// The github user agent header must be of the form GitHub-Hookshot/xxx
/// 
/// If the user agent is provided (GitHub-Hookshot) then hmac verification
/// takes place
/// 
/// The hmac provided by the header x-hub-signature-256, is checked against 
/// the GithubConfig.token value and the bodies bytes
/// 
/// The body is only read after the user agent matches
/// 
/// # Example
/// 
/// ```rust
/// use std::net::{IpAddr, Ipv4Addr, SocketAddr};
/// use std::sync::{Arc, Mutex};
///
/// use axum::
/// {
///     routing::post, 
///     Router, 
///     middleware
/// };
/// 
/// use pulse::web::request::discord::model::Webhook;
/// use pulse::web::response::github::{filter_github, GithubConfig};
/// 
/// pub async fn server() {
/// 
///     let github = GithubConfig::new("secret".to_string(), Webhook::new("url".to_string()));
/// 
///     let app = Router::new()
///     .route("/", post(|| async move {  }))
///     .layer(middleware::from_fn_with_state(github, filter_github));
/// 
///     let ip = Ipv4Addr::new(127,0,0,1);
///     let addr = SocketAddr::new(IpAddr::V4(ip), 3000);
/// 
///     axum::Server::bind(&addr)
///     .serve(app.into_make_service_with_connect_info::<SocketAddr>())
///     .await
///     .unwrap();
/// 
/// }
/// ````
pub async fn filter_github<B>
(
    State(app_state): State<GithubConfig>,
    headers: HeaderMap,
    request: Request<B>,
    next: Next<B>
) -> Result<Response, StatusCode>
where B: axum::body::HttpBody<Data = Bytes>
{

    let user_agent = match std::str::from_utf8(headers["user-agent"].as_bytes())
    {
        Ok(u) => u,
        Err(_) =>
        {
            crate::debug("no/mangled user agent".to_string(), None);
            return Ok(next.run(request).await)
        }
    };

    match Regex::new(r"GitHub-Hookshot").unwrap().captures(user_agent)
    {
        Some(_) => {crate::debug("github user agent, processing".to_string(), None);},
        None => 
        {
            crate::debug("not github user".to_string(), None);
            return Ok(next.run(request).await)
        }
    }

    let body = request.into_body();
    let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(_) => {
            return Err(StatusCode::BAD_REQUEST)
        }
    };

    return match github_verify(app_state.clone(), headers, bytes.clone()).await
    {
        StatusCode::ACCEPTED => 
        {
            Ok(github_release(app_state, bytes).await.into_response())
        },
        r => {Ok(r.into_response())}
    }

    
}

/// Verify a github POST request
/// 
/// Checks (hmac) the header x-hub-signature-256 comparing to the local token
/// app_state.token with the passes body bytes
/// 
async fn github_verify
(
    app_state: GithubConfig,
    headers: HeaderMap,
    body: Bytes
) -> StatusCode
{

    match headers.contains_key("x-hub-signature-256")
    {
        false => 
        {
            crate::debug("no signature".to_string(), None);
            return StatusCode::UNAUTHORIZED
        },
        true => {}
    };

    let signature = match std::str::from_utf8(headers["x-hub-signature-256"].as_bytes())
    {
        Ok(s) => s,
        Err(_) => 
        {
            crate::debug("signature utf8 parse failure".to_string(), None);
            return StatusCode::BAD_REQUEST
        }
    };

    let post_digest = Regex::new(r"sha256=").unwrap().replace_all(signature, "").into_owned().to_uppercase();

    let token = app_state.token.clone();
    let key = match PKey::hmac(token.as_bytes())
    {
        Ok(k) => k,
        Err(_) => 
        {
            crate::debug("key creation failure".to_string(), None);
            return StatusCode::INTERNAL_SERVER_ERROR
        }
    };

    let mut signer = match Signer::new(MessageDigest::sha256(), &key)
    {
        Ok(k) => k,
        Err(_) => 
        {
            crate::debug("signer creation failure".to_string(), None);
            return StatusCode::INTERNAL_SERVER_ERROR
        }
    };
    
    match signer.update(&body)
    {
        Ok(k) => k,
        Err(_) => 
        {
            crate::debug("signing update failure".to_string(), None);
            return StatusCode::INTERNAL_SERVER_ERROR
        }
    };

    let hmac = match signer.sign_to_vec()
    {
        Ok(k) => k,
        Err(_) => 
        {
            crate::debug("sign failure".to_string(), None);
            return StatusCode::INTERNAL_SERVER_ERROR
        }
    };

    crate::debug(format!("post_digtest: {}, len: {}\nlocal hmac: {}, len: {}", post_digest, post_digest.len(), dump_bytes(&hmac), dump_bytes(&hmac).len()), None);

    match memcmp::eq(&hmac, &read_bytes(post_digest.clone()))
    {
        true => {},
        false => 
        {
            crate::debug(format!("bad signature: local/post\n{}\n{}", post_digest, dump_bytes(&hmac)), None);
            return StatusCode::UNAUTHORIZED
        }
    }

    // it is now safe to process the POST request

    let body = std::str::from_utf8(&body).unwrap().to_string();

    crate::debug(format!("[{}] Got request:\n\nheader:\n\n{:?}\n\nbody:\n\n{}", Local::now(), headers, body), None);
    
    StatusCode::ACCEPTED
   
}

/// Send format a message to send as a POST request to discord
/// 
async fn github_release
(
    app_state: GithubConfig,
    body: Bytes
) -> StatusCode
{

    let sbody = std::str::from_utf8(&body).unwrap().to_string();

    let parsed_data: HashMap<String, serde_json::Value> = match serde_json::from_str(&strip_control_characters(sbody))
    {
        Ok(d) => d,
        Err(e) => 
        {
            crate::debug(format!("error parsing body: {}", e), None);
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    if parsed_data.contains_key("action") 
    {
        let action: GithubReleaseActionType = match parsed_data["action"].to_owned().as_str()
        {
            Some(s) => {s.into()},
            None => 
            {
                crate::debug(format!("action could not be parsed \n\nGot:\n {:?}", parsed_data["action"]), None);
                return StatusCode::BAD_REQUEST;
            }
        };

        return respond(action, parsed_data, app_state.discord).await;
    }
    else
    {
        crate::debug(format!("no action entry in JSON payload \n\nGot:\n {:?}", parsed_data), None);
        return StatusCode::BAD_REQUEST;
    }

}

async fn respond(action: GithubReleaseActionType, data: HashMap<String, serde_json::Value>, disc: Webhook) -> StatusCode
{
    crate::debug(format!("Processing github release payload: {:?}", action), None);

    match action 
    {
        GithubReleaseActionType::CREATED => {}
        GithubReleaseActionType::PUBLISHED => {},
        _ => {return StatusCode::OK}
    };

    let msg = format!
    (
        "New release just dropped!\n  {} just got a newly {} release. \n\n  Check it out here: {}", 
        data["repository"]["name"], 
        Into::<String>::into(action),
        data["release"]["url"]
    );

    match post(disc, msg).await
    {
        Ok(_) => StatusCode::OK,
        Err(e) => 
        {
            crate::debug(format!("error while sending to discord {}", e), None);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
    
}