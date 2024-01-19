use std::collections::HashMap;

use axum::http::{HeaderMap, StatusCode};
use axum::body::Bytes;

use crate::web::discord;
use crate::web::event::{EventConfig, read_config, expand_template, select_template};
use crate::web::event::Event;

use super::github_filter::github_request_is_authentic;

pub const X_GTIHUB_EVENT: &str = "push";

pub struct GithubPushed 
{
    config: EventConfig
}

impl GithubPushed
{
    pub fn new() -> GithubPushed
    {
        GithubPushed { config: EventConfig::new() }
    }
}

impl Event for GithubPushed
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
        self.config = read_config("github_pushed");
    }

    fn is_authentic(&self, headers: HeaderMap, body: Bytes) -> StatusCode
    {
        return github_request_is_authentic(self.get_token(), headers, body);
    }

    fn into_response(&self, data: HashMap<String, serde_json::Value>) -> (Option<String>, StatusCode)
    {
        let name = match data["repository"]["name"].as_str()
        {
            Some(s) => s,
            None => {return (None, StatusCode::INTERNAL_SERVER_ERROR)}
        };
    
        crate::debug(format!("got a push to {}", name), None);

        let template = select_template(self.config.get_templates(), data.clone());

        (expand_template(template, data), StatusCode::OK)
    }
}