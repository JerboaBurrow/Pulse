use crate::web::request::discord::model::Webhook;

#[derive(Clone)]
pub struct GithubConfig
{
    token: String,
    discord: Webhook
}

impl GithubConfig
{
    pub fn new(t: String, w: Webhook) -> GithubConfig
    {
        GithubConfig {token: t, discord: w}
    } 

    pub fn get_token(self: GithubConfig) -> String
    {
        self.token
    }

    pub fn get_webhook(self: GithubConfig) -> Webhook
    {
        self.discord
    }
}

#[derive(Debug)]
pub enum GithubReleaseActionType
{
    CREATED,
    DELETED,
    EDITED,
    PRERELEASED,
    PUBLISHED,
    RELEASED,
    UNPUBLISHED,
    UNKOWN
}

impl From<&str> for GithubReleaseActionType
{
    
    fn from(s: &str) -> GithubReleaseActionType
    {
        match s 
        {
            "created" => Self::CREATED,
            "deleted" => Self::DELETED,
            "edited" => Self::EDITED,
            "prereleased" => Self::PRERELEASED,
            "published" => Self::PUBLISHED,
            "released" => Self::RELEASED,
            "unpublished" => Self::UNPUBLISHED,
            _ => Self::UNKOWN
        }
    }
}

impl Into<String> for GithubReleaseActionType
{
    fn into(self: GithubReleaseActionType) -> String 
    {
        match self
        {
            GithubReleaseActionType::CREATED => "created".to_string(),
            GithubReleaseActionType::DELETED => "deleted".to_string(),
            GithubReleaseActionType::EDITED => "edited".to_string(),
            GithubReleaseActionType::PRERELEASED => "prereleased".to_string(),
            GithubReleaseActionType::PUBLISHED => "published".to_string(),
            GithubReleaseActionType::RELEASED => "released".to_string(),
            GithubReleaseActionType::UNPUBLISHED => "unpublished".to_string(),
            _ => "unkown".to_string()
        }
    }
}

#[derive(Debug)]
pub enum GithubStarredActionType
{
    CREATED,
    DELETED,
    UNKOWN
}

impl From<&str> for GithubStarredActionType
{
    fn from(s: &str) -> GithubStarredActionType
    {
        match s 
        {
            "created" => Self::CREATED,
            "deleted" => Self::DELETED,
            _ => Self::UNKOWN
        }
    }
}

impl Into<String> for GithubStarredActionType
{
    fn into(self: GithubStarredActionType) -> String 
    {
        match self
        {
            GithubStarredActionType::CREATED => "created".to_string(),
            GithubStarredActionType::DELETED => "deleted".to_string(),
            _ => "unkown".to_string()
        }
    }
}