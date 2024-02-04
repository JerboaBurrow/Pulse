mod common;

#[cfg(test)]
mod test_fork_payload
{
    use std::collections::HashMap;

    use regex::Regex;

    use crate::common::FORKED_PAYLOAD;

    #[test]
    fn deserialise_json()
    {
        let parsed_data: HashMap<String, serde_json::Value> = serde_json::from_str(FORKED_PAYLOAD).unwrap();
        
        assert_eq!(parsed_data["forkee"]["name"], "Pulse");

        assert_eq!(parsed_data["repository"]["name"], "Pulse");
    }

}