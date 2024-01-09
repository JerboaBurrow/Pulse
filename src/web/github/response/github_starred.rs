use std::collections::HashMap;

use axum::http::{HeaderMap, StatusCode};
use axum::body::Bytes;

use crate::web::discord;
use crate::web::event::{EventConfig, read_config};
use crate::web::event::Event;

use super::github_filter::github_request_is_authentic;
use super::model::GithubStarredActionType;

pub const X_GTIHUB_EVENT: &str = "star";

pub struct GithubStarred
{
    config: EventConfig
}

impl GithubStarred
{
    pub fn new() -> GithubStarred
    {
        GithubStarred { config: EventConfig::new() }
    }
}

impl Event for GithubStarred
{
    fn get_token(&self) -> String
    {
        self.config.get_token()
    }

    fn get_end_point(&self) -> discord::request::model::Webhook
    {
        self.config.get_end_point()
    }

    fn load_config(&mut self)
    {
        self.config = read_config("github_starred");
    }

    fn is_authentic(&self, headers: HeaderMap, body: Bytes) -> StatusCode
    {
        return github_request_is_authentic(self.get_token(), headers, body);
    }

    fn into_response(&self, data: HashMap<String, serde_json::Value>) -> (Option<String>, StatusCode)
    {

        let action: GithubStarredActionType = if data.contains_key("action") 
        {
            if data.contains_key("starred_at")
            {
                match data["action"].to_owned().as_str()
                {
                    Some(s) => {s.into()},
                    None => 
                    {
                        crate::debug(format!("action could not be parsed \n\nGot:\n {:?}", data["action"]), None);
                        return (None, StatusCode::BAD_REQUEST);
                    }
                }
            }
            else
            {
                return (None, StatusCode::BAD_REQUEST);
            }
        }
        else
        {
            return (None, StatusCode::BAD_REQUEST);
        };

        match action 
        {
            GithubStarredActionType::UNKOWN => {return (None, StatusCode::BAD_REQUEST)}
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
            return (None, StatusCode::INTERNAL_SERVER_ERROR)
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
            _ => {return (None, StatusCode::INTERNAL_SERVER_ERROR)}
        };
    
        crate::debug(format!("Formatted message {:?}", msg), None);
    
        if crate::DONT_MESSAGE_ON_PRIVATE_REPOS && data["repository"]["private"].as_bool().is_some_and(|x|x)
        {
            return (None, StatusCode::OK);
        }

        (Some(msg), StatusCode::OK)
    }
}