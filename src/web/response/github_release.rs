//! Github release response

use std::collections::HashMap;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;

use axum::response::Response;

use std::convert::{From, Into};

use chrono::Local;
use regex::Regex;

fn strip_control_characters(s: String) -> String
{
    let re = Regex::new(r"[\u0000-\u001F]").unwrap().replace_all(&s, "");
    return re.to_string()
}

#[derive(Debug)]
enum GITHUB_RELEASE_ACTION_TYPE
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

impl From<&str> for GITHUB_RELEASE_ACTION_TYPE
{
    fn from(s: &str) -> GITHUB_RELEASE_ACTION_TYPE
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
    State(body): State<String>
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
        let action: GITHUB_RELEASE_ACTION_TYPE = match parsed_data["action"].to_owned().as_str()
        {
            Some(s) => {s.into()},
            None => 
            {
                crate::debug(format!("action could not be parsed \n\nGot:\n {:?}", parsed_data["action"]), None);
                return Ok((StatusCode::BAD_REQUEST).into_response());
            }
        };

        return respond(action, parsed_data).await;
    }
    else
    {
        crate::debug(format!("no action entry in JSON payload \n\nGot:\n {:?}", parsed_data), None);
        return Ok((StatusCode::BAD_REQUEST).into_response());
    }

}

async fn respond(action: GITHUB_RELEASE_ACTION_TYPE, data: HashMap<String, serde_json::Value>) -> Result<Response, StatusCode>
{
    crate::debug(format!("Processing github release payload: {:?}", action), None);
    
    match action 
    {
        GITHUB_RELEASE_ACTION_TYPE::CREATED => {}
        GITHUB_RELEASE_ACTION_TYPE::PUBLISHED => {},
        _ => {}
    }

    Ok((StatusCode::OK).into_response())
}