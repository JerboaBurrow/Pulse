use std::collections::HashMap;

use serde::{Serialize, Deserialize};

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