//! Github release response

use std::collections::HashMap;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;

use axum::response::Response;

use std::convert::{From, Into};

use regex::Regex;

use crate::discord::model::Webhook;

fn strip_control_characters(s: String) -> String
{
    let re = Regex::new(r"[\u0000-\u001F]").unwrap().replace_all(&s, "");
    return re.to_string()
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

pub async fn github_release<B>
(
    State(body): State<String>,
    State(disc): State<Webhook>
) -> Result<Response, StatusCode>
{

    let parsed_data: HashMap<String, serde_json::Value> = match serde_json::from_str(&strip_control_characters(body))
    {
        Ok(d) => d,
        Err(e) => 
        {
            crate::debug(format!("error parsing body: {}", e), None);
            return Ok((StatusCode::INTERNAL_SERVER_ERROR).into_response());
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
                return Ok((StatusCode::BAD_REQUEST).into_response());
            }
        };

        return respond(action, parsed_data, disc).await;
    }
    else
    {
        crate::debug(format!("no action entry in JSON payload \n\nGot:\n {:?}", parsed_data), None);
        return Ok((StatusCode::BAD_REQUEST).into_response());
    }

}

async fn respond(action: GithubReleaseActionType, data: HashMap<String, serde_json::Value>, disc: Webhook) -> Result<Response, StatusCode>
{
    crate::debug(format!("Processing github release payload: {:?}", action), None);
    
    match action 
    {
        GithubReleaseActionType::CREATED => {}
        GithubReleaseActionType::PUBLISHED => {},
        _ => {}
    }

    Ok((StatusCode::OK).into_response())
}