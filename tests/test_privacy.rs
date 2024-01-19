mod common;

#[cfg(test)]
mod test_privacy
{
    use std::collections::HashMap;

    use pulse::web::{github::response::{github_release::GithubReleased, github_pushed::GithubPushed, github_starred::GithubStarred}, event::Event};
    use regex::Regex;

    use crate::common::{RELEASE_PAYLOAD, RELEASE_PAYLOAD_PRIVATE};

    fn strip_control_characters(s: String) -> String
    {
        let re = Regex::new(r"[\u0000-\u001F]").unwrap().replace_all(&s, "");
        return re.to_string()
    }

    #[test]
    fn release_privacy()
    {
        let parsed_data: HashMap<String, serde_json::Value> = serde_json::from_str(&strip_control_characters(RELEASE_PAYLOAD_PRIVATE.to_string())).unwrap();
        
        assert_eq!(parsed_data["repository"]["private"], true);

        let mut action = GithubReleased::new();

        action.load_config();

        let action_response = action.into_response(parsed_data);

        assert!(action_response.0.is_none());
    }

    #[test]
    fn push_privacy()
    {
        let parsed_data: HashMap<String, serde_json::Value> = serde_json::from_str(&strip_control_characters(RELEASE_PAYLOAD_PRIVATE.to_string())).unwrap();
        
        assert_eq!(parsed_data["repository"]["private"], true);

        let mut action = GithubPushed::new();

        action.load_config();

        let action_response = action.into_response(parsed_data);

        assert!(action_response.0.is_none());
    }

    #[test]
    fn star_privacy()
    {
        let parsed_data: HashMap<String, serde_json::Value> = serde_json::from_str(&strip_control_characters(RELEASE_PAYLOAD_PRIVATE.to_string())).unwrap();
        
        assert_eq!(parsed_data["repository"]["private"], true);

        let mut action = GithubStarred::new();

        action.load_config();

        let action_response = action.into_response(parsed_data);

        assert!(action_response.0.is_none());
    }

}