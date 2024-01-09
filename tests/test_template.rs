mod common;

#[cfg(test)]
mod test_template
{
    use std::collections::HashMap;
    use serde_json;

    use crate::common::ISSUE_PAYLOAD;
    use pulse::web::event;

    const ISSUE_TEMPLATE_OK: &str = "this is an issue template string the action was <action> the repository url was <issue/repository_url>";
    const ISSUE_TEMPLATE_BAD: &str = "this is an issue template string the action was <action> the repository url was <issue/a_non_existant_value>";

    #[test]
    fn issue_format_ok()
    {
        let parsed_data: HashMap<String, serde_json::Value> = serde_json::from_str(ISSUE_PAYLOAD).unwrap();

        assert!(parsed_data["issue"].pointer("/repository_url").is_some());
        
        let formatted = event::expand_template(ISSUE_TEMPLATE_OK.to_string(), parsed_data).unwrap();
        assert_eq!(formatted, "this is an issue template string the action was opened the repository url was https://api.github.com/repos/JerboaBurrow/test".to_string())
    }

    #[test]
    fn issue_format_bad()
    {
        let parsed_data: HashMap<String, serde_json::Value> = serde_json::from_str(ISSUE_PAYLOAD).unwrap();

        assert!(parsed_data["issue"].pointer("/a_non_existant_value").is_none());
        
        let formatted = event::expand_template(ISSUE_TEMPLATE_BAD.to_string(), parsed_data).unwrap();
        assert_eq!(formatted, "this is an issue template string the action was opened the repository url was <issue/a_non_existant_value>".to_string())
    }
}