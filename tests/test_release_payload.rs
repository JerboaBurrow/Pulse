mod common;

#[cfg(test)]
mod test_release_payload
{
    use std::collections::HashMap;

    use regex::Regex;

    use crate::common::RELEASE_PAYLOAD;

    fn strip_control_characters(s: String) -> String
    {
        let re = Regex::new(r"[\u0000-\u001F]").unwrap().replace_all(&s, "");
        return re.to_string()
    }

    #[test]
    fn deserialise_json()
    {
        let parsed_data: HashMap<String, serde_json::Value> = serde_json::from_str(&strip_control_characters(RELEASE_PAYLOAD.to_string())).unwrap();
        
        assert_eq!(parsed_data["action"], "published");

        assert_eq!(parsed_data["repository"]["name"], "jGL");
    }

}