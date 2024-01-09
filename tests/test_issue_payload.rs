use serde_json;

#[cfg(test)]
mod common;

mod test_issue_payload
{
    use std::collections::HashMap;

    use crate::common::ISSUE_PAYLOAD;

    #[test]
    fn deserialise_json()
    {
        let parsed_data: HashMap<String, serde_json::Value> = serde_json::from_str(ISSUE_PAYLOAD).unwrap();
        
        assert_eq!(parsed_data["action"], "opened");

        assert_eq!(parsed_data["issue"]["repository_url"], "https://api.github.com/repos/JerboaBurrow/test");
    }

}