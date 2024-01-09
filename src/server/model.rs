use crate::web::github::model::GithubStats;

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