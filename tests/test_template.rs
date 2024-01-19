mod common;

#[cfg(test)]
mod test_template
{
    use std::collections::HashMap;
    use serde_json;

    use crate::common::{ISSUE_PAYLOAD, STAR_CREATED_PAYLOAD, STAR_DELETED_PAYLOAD};
    use pulse::web::event::{self, Template, select_template};

    const ISSUE_TEMPLATE_OK: &str = "this is an issue template string the action was <action> the repository url was <issue/repository_url>";
    const ISSUE_TEMPLATE_BAD: &str = "this is an issue template string the action was <action> the repository url was <issue/a_non_existant_value>";

    const SINGLE_JSON_TEMPLATE: &str = r#"
    [ 
            {
                "check_value_path": "",
                "check_value": "",
                "body": "New release!"
            }
    ]
    "#;

    const EMPTY_JSON_TEMPLATE: &str = r#"
    [ 
    ]
    "#;

    const STAR_JSON_TEMPLATE: &str = r#"
    [
        {
            "check_value_path": "action",
            "check_value": "created",
            "body": "New star!"
        },
        {
            "check_value_path": "action",
            "check_value": "deleted",
            "body": "Lost a star!"
        }
    ]
    "#;

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

    #[test]
    fn single_json_templates()
    {
        let templates: Vec<Template> = serde_json::from_str(SINGLE_JSON_TEMPLATE).unwrap();

        let parsed_data: HashMap<String, serde_json::Value> = serde_json::from_str(ISSUE_PAYLOAD).unwrap();

        let selected = select_template(templates, parsed_data);

        assert_eq!(selected, "New release!".to_string())
    }

    #[test]
    fn empty_json_templates()
    {
        let templates: Vec<Template> = serde_json::from_str(EMPTY_JSON_TEMPLATE).unwrap();

        let parsed_data: HashMap<String, serde_json::Value> = serde_json::from_str(ISSUE_PAYLOAD).unwrap();

        let selected = select_template(templates, parsed_data);

        assert_eq!(selected, "".to_string())
    }

    #[test]
    fn select_star_created_json_templates()
    {
        let templates: Vec<Template> = serde_json::from_str(STAR_JSON_TEMPLATE).unwrap();

        let parsed_data: HashMap<String, serde_json::Value> = serde_json::from_str(STAR_CREATED_PAYLOAD).unwrap();

        let selected = select_template(templates, parsed_data);

        assert_eq!(selected, "New star!".to_string())
    }

    #[test]
    fn select_star_deleted_json_templates()
    {
        let templates: Vec<Template> = serde_json::from_str(STAR_JSON_TEMPLATE).unwrap();

        let parsed_data: HashMap<String, serde_json::Value> = serde_json::from_str(STAR_DELETED_PAYLOAD).unwrap();

        let selected = select_template(templates, parsed_data);

        assert_eq!(selected, "Lost a star!".to_string())
    }

    #[test]
    fn select_path_not_found()
    {
        let templates: Vec<Template> = serde_json::from_str(STAR_JSON_TEMPLATE).unwrap();

        let parsed_data: HashMap<String, serde_json::Value> = serde_json::from_str(ISSUE_PAYLOAD).unwrap();

        let selected = select_template(templates, parsed_data);

        assert_eq!(selected, "".to_string())
    }
}