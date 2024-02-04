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
use regex::Regex;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::server::model::AppState;
use crate::stats::io::collect;
use crate::util::strip_control_characters;

use crate::web::discord::request::post::post;
use crate::web::event::Event;
use crate::web::github::
response::
{
    github_release, 
    github_starred,
    github_pushed,
};

use crate::web::is_authentic;

use super::{github_forked, github_ping};

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
/// use std::sync::Arc;
/// use tokio::sync::Mutex;
///
/// use axum::
/// {
///     routing::post, 
///     Router, 
///     middleware
/// };
/// 
/// use pulse::web::
/// {
///    throttle::{IpThrottler, handle_throttle},
///    github::{response::github_filter::filter_github, model::GithubStats},
///    discord::request::model::Webhook
/// };
/// 
/// use pulse::server::model::AppState;
/// 
/// pub async fn server() {
/// 
///     let github = Arc::new(Mutex::new(AppState::new(GithubStats::new())));
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
    State(app_state): State<Arc<Mutex<AppState>>>,
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

    let mut event: Box<dyn Event + Send> = match headers.contains_key("x-gitHub-event")
    {
        true => 
        {
            match std::str::from_utf8(headers["x-github-event"].as_bytes()).unwrap()
            {
                github_pushed::X_GTIHUB_EVENT => 
                {
                    Box::new(github_pushed::GithubPushed::new())
                },
                github_release::X_GTIHUB_EVENT =>
                {
                    Box::new(github_release::GithubReleased::new())
                },
                github_starred::X_GTIHUB_EVENT =>
                {
                    Box::new(github_starred::GithubStarred::new())
                },
                github_ping::X_GTIHUB_EVENT => 
                {
                    Box::new(github_ping::GithubPing::new())
                },
                github_forked::X_GTIHUB_EVENT =>
                {
                    Box::new(github_forked::GithubForked::new())
                }
                _ => return Ok(StatusCode::CONTINUE.into_response())
            }
        },
        false => return Ok(StatusCode::BAD_REQUEST.into_response())
    };

    event.load_config();

    match event.is_authentic(headers, bytes.clone())
    {
        StatusCode::ACCEPTED => 
        {
            let body = std::str::from_utf8(&bytes).unwrap().to_string();

            let parsed_data: HashMap<String, serde_json::Value> = match serde_json::from_str(&strip_control_characters(body))
            {
                Ok(d) => d,
                Err(e) => 
                {
                    crate::debug(format!("error parsing body: {}", e), None);
                    return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response());
                }
            };

            collect(app_state, parsed_data.clone()).await;

            let response = event.into_response(parsed_data.clone());

            if response.1 != StatusCode::OK
            {
                return Ok(response.1.into_response())
            }

            let status = match response.0
            {
                Some(msg) =>
                {
                    match post(event.get_end_point(), msg).await
                    {
                        Ok(_) => StatusCode::OK,
                        Err(e) => 
                        {
                            crate::debug(format!("error while sending to discord {}", e), None);
                            StatusCode::INTERNAL_SERVER_ERROR
                        }
                    }
                },
                None => response.1
            };

            return Ok(status.into_response())
        },
        s => return Ok(s.into_response())
    }
    
}

/// Verify a github POST request
/// 
/// Checks (hmac) the header x-hub-signature-256 comparing to the local token
/// app_state.token with the passes body bytes
/// 
pub fn github_request_is_authentic
(
    token: String,
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

    match is_authentic(token, signature.to_owned(), body.clone())
    {
        StatusCode::ACCEPTED => {},
        s => {return s}
    };

    crate::debug(format!("[{}] Got request:\n\nheader:\n\n{:?}\n\nbody:\n\n{:?}", Local::now(), headers, body), None);
    
    StatusCode::ACCEPTED
   
}

