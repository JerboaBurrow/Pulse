use std::collections::HashMap;

use axum::http::{HeaderMap, StatusCode};
use axum::body::Bytes;

use crate::web::discord;
use crate::web::event::{EventConfig, read_config, select_template, expand_template};
use crate::web::event::Event;

use super::github_filter::github_request_is_authentic;

pub const X_GTIHUB_EVENT: &str = "issues";

pub struct GithubIssue
{
    config: EventConfig
}

impl GithubIssue
{
    pub fn new() -> GithubIssue
    {
        GithubIssue { config: EventConfig::new() }
    }
}

impl Event for GithubIssue
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
        self.config = read_config("github_issue");
    }

    fn is_authentic(&self, headers: HeaderMap, body: Bytes) -> StatusCode
    {
        return github_request_is_authentic(self.get_token(), headers, body);
    }

    fn into_response(&self, data: HashMap<String, serde_json::Value>) -> (Option<String>, StatusCode)
    {

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

        crate::debug(format!("issue from {:?}", data["sender"]["login"]), None);
    
        if self.config.silent_on_private_repos() && data["repository"]["private"].as_bool().is_some_and(|x|x)
        {
            return (None, StatusCode::OK);
        }

        (expand_template(template, data), StatusCode::OK)
    }
}