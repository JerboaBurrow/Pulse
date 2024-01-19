use std::collections::HashMap;

use axum::http::{HeaderMap, StatusCode};
use axum::body::Bytes;
use regex::Regex;

use crate::web::discord;
use crate::web::event::{EventConfig, read_config, expand_template, select_template};
use crate::web::event::Event;

use super::github_filter::github_request_is_authentic;
use super::model::GithubReleasedActionType;

pub const X_GTIHUB_EVENT: &str = "release";

pub struct GithubReleased
{
    config: EventConfig
}

impl GithubReleased
{
    pub fn new() -> GithubReleased
    {
        GithubReleased { config: EventConfig::new() }
    }
}

impl Event for GithubReleased
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
        self.config = read_config("github_released");
    }

    fn is_authentic(&self, headers: HeaderMap, body: Bytes) -> StatusCode
    {
        return github_request_is_authentic(self.get_token(), headers, body);
    }

    fn into_response(&self, data: HashMap<String, serde_json::Value>) -> (Option<String>, StatusCode)
    {

        let action: GithubReleasedActionType = if data.contains_key("action") 
        {
            if data.contains_key("release")
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
            GithubReleasedActionType::CREATED => {}
            GithubReleasedActionType::PUBLISHED => {},
            GithubReleasedActionType::UNKOWN => {return (None, StatusCode::BAD_REQUEST)}
            _ => {return (None, StatusCode::OK)}
        };

        let template = select_template(self.config.get_templates(), data.clone());

        let template = if data["repository"]["name"].is_string()
        {
            match data["repository"]["name"] == "Pulse" 
            {
               true => template.replacen("<repository/name>", "Pulse (that's me!)", 1),
               false => {template}
            }
        }
        else
        {
            return (None, StatusCode::INTERNAL_SERVER_ERROR)
        };
    
        if crate::DONT_MESSAGE_ON_PRIVATE_REPOS && data["repository"]["private"].as_bool().is_some_and(|x|x)
        {
            return (None, StatusCode::OK);
        }

        (expand_template(template, data), StatusCode::OK)
    }
}