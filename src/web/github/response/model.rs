#[derive(Debug)]
pub enum GithubReleasedActionType
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

impl From<&str> for GithubReleasedActionType
{
    
    fn from(s: &str) -> GithubReleasedActionType
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

impl Into<String> for GithubReleasedActionType
{
    fn into(self: GithubReleasedActionType) -> String 
    {
        match self
        {
            GithubReleasedActionType::CREATED => "created".to_string(),
            GithubReleasedActionType::DELETED => "deleted".to_string(),
            GithubReleasedActionType::EDITED => "edited".to_string(),
            GithubReleasedActionType::PRERELEASED => "prereleased".to_string(),
            GithubReleasedActionType::PUBLISHED => "published".to_string(),
            GithubReleasedActionType::RELEASED => "released".to_string(),
            GithubReleasedActionType::UNPUBLISHED => "unpublished".to_string(),
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