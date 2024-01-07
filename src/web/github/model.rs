use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::web::discord::request::model::Webhook;

#[derive(Clone, Serialize, Deserialize)]
pub struct GithubRepoStats
{
    pub stars: u64,
    pub pushes: u64,
}

impl GithubRepoStats
{
    pub fn new() -> GithubRepoStats
    {
        GithubRepoStats {stars: 0, pushes: 0}
    }

    pub fn update(&mut self, stats: GithubRepoStats)
    {
        self.stars = stats.stars;
        self.pushes += stats.pushes;
    }

    pub fn clear(&mut self)
    {
        self.pushes = 0;
        self.stars = 0;
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GithubStats
{
    pub repos: HashMap<String, GithubRepoStats>
}

impl GithubStats
{
    pub fn new() -> GithubStats
    {
        GithubStats {repos: HashMap::new()}
    }
}

#[derive(Clone)]
pub struct GithubConfig
{
    token: String,
    discord: Webhook,
    stats: GithubStats
}

impl GithubConfig
{
    pub fn new(t: String, w: Webhook, s: GithubStats) -> GithubConfig
    {
        GithubConfig {token: t, discord: w, stats: s}
    } 

    pub fn get_token(&self) -> String
    {
        String::from(self.token.clone())
    }

    pub fn get_webhook(&self) -> Webhook
    {
        self.discord.clone()
    }

    pub fn get_stats(&mut self) -> &mut GithubStats
    {
        &mut self.stats
    }
}