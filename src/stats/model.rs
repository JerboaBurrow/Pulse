use serde::{Serialize, Deserialize};

pub const STATS_CONFIG_PATH: &str = "stats.config";

#[derive(Clone, Serialize, Deserialize)]
pub struct StatsConfig
{
    #[serde(default="default_privacy")]
    dont_track_private_repos: bool
}

fn default_privacy() -> bool
{
    true
}

impl StatsConfig
{
    pub fn suppress_private(&self) -> bool
    {
        self.dont_track_private_repos
    }

    pub fn new(dont_track_private: bool) -> StatsConfig
    {
        StatsConfig { dont_track_private_repos: dont_track_private }
    }
}