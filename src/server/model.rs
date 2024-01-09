use serde::{Serialize, Deserialize};

use crate::web::{github::model::GithubStats, discord::request::model::Webhook};

pub const CONFIG_PATH: &str = "config.json";

#[derive(Clone, Serialize, Deserialize)]
pub struct Config
{
    port: u16,
    stats_endpoint: Webhook,
    cert_path: String,
    key_path: String
}

impl Config 
{
    pub fn get_port(&self) -> u16
    {
        self.port
    }

    pub fn get_end_point(&self) -> Webhook
    {
        self.stats_endpoint.clone()
    }

    pub fn get_cert_path(&self) -> String
    {
        self.cert_path.clone()
    }

    pub fn get_key_path(&self) -> String
    {
        self.key_path.clone()
    }
    
}

#[derive(Clone)]
pub struct AppState
{
    github_stats: GithubStats
}

impl AppState
{
    pub fn new(s: GithubStats) -> AppState
    {
        AppState {github_stats: s}
    } 

    pub fn get_github_stats(&mut self) -> &mut GithubStats
    {
        &mut self.github_stats
    }
}