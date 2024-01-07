use std::collections::HashMap;

use std::sync::Arc;
use tokio::sync::Mutex;

use reqwest::StatusCode;

use crate::{web::{discord::request::post::post, github::model::GithubConfig}, stats::io::collect};

use super::model::GithubStarredActionType;

pub async fn respond_starred(action: GithubStarredActionType, data: HashMap<String, serde_json::Value>, app_state: Arc<Mutex<GithubConfig>>) -> StatusCode
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

    crate::debug(format!("Extracted name {:?}", name), None);

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

    crate::debug(format!("Formatted message {:?}", msg), None);

    collect(app_state.clone(), data.clone()).await;

    if crate::DONT_MESSAGE_ON_PRIVATE_REPOS && data["repository"]["private"].as_bool().is_some_and(|x|x)
    {
        return StatusCode::OK;
    }

    match post(app_state.lock().await.get_webhook(), msg).await
    {
        Ok(_) => StatusCode::OK,
        Err(e) => 
        {
            crate::debug(format!("error while sending to discord {}", e), None);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
    
}