use std::collections::HashMap;

use std::sync::Arc;
use tokio::sync::Mutex;

use reqwest::StatusCode;

use crate::{web::github::model::GithubConfig, stats::io::collect};

pub async fn respond_pushed(data: HashMap<String, serde_json::Value>, app_state: Arc<Mutex<GithubConfig>>) -> StatusCode
{
    let name = match data["repository"]["name"].as_str()
    {
        Some(s) => s,
        None => {return StatusCode::INTERNAL_SERVER_ERROR}
    };

    crate::debug(format!("got a push to {}", name), None);

    collect(app_state, data).await;

    StatusCode::OK
    
}