use std::collections::HashMap;

use reqwest::StatusCode;

use crate::web::request::discord::{model::Webhook, post::post};

use super::model::GithubReleaseActionType;

pub async fn respond_release(action: GithubReleaseActionType, data: HashMap<String, serde_json::Value>, disc: Webhook) -> StatusCode
{
    crate::debug(format!("Processing github release payload: {:?}", action), None);

    match action 
    {
        GithubReleaseActionType::CREATED => {}
        GithubReleaseActionType::PUBLISHED => {},
        _ => {return StatusCode::OK}
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

    let msg = format!
    (
        "New release just dropped!\n  {} just got a newly {} release :confetti_ball: \n  Check it out here: {}", 
        name,
        Into::<String>::into(action),
        data["release"]["html_url"].as_str().unwrap()
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