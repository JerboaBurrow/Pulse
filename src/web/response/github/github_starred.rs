use std::collections::HashMap;

use reqwest::StatusCode;

use crate::web::request::discord::{model::Webhook, post::post};

use super::model::GithubStarredActionType;

pub async fn respond_starred(action: GithubStarredActionType, data: HashMap<String, serde_json::Value>, disc: Webhook) -> StatusCode
{
    crate::debug(format!("Processing github starred payload: {:?}", action), None);

    match action 
    {
        GithubStarredActionType::UNKOWN => {return StatusCode::BAD_REQUEST}
        _ => {}
    };

    let name = if data["repository"]["name"].is_string()
    {
         match data["repository"]["name"] == "Pulse" 
        {
            true => "Pulse (that's me!)",
            false => data["repository"]["name"].as_str().unwrap()
        }
    }
    else
    {
        return StatusCode::INTERNAL_SERVER_ERROR
    };

    let msg = match action
    {
        GithubStarredActionType::CREATED =>
        {
            format!
            (
                "{} just got a new :star:, that makes {}", 
                name, 
                data["repository"]["stargazers_count"]
            )
        },
        GithubStarredActionType::DELETED =>
        {
            format!
            (
                "{} just lost a :star:, that makes {} :cry:", 
                name, 
                data["repository"]["stargazers_count"]
            )
        },
        _ => {return StatusCode::INTERNAL_SERVER_ERROR}
    };

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