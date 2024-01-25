use std::collections::HashMap;

use axum::http::{StatusCode, HeaderMap};
use axum::body::Bytes;

use regex::Regex;
use serde::{Serialize, Deserialize};

use crate::web::discord;

use std::path::Path;
use crate::util::read_file_utf8;

pub const CONFIG_PATH: &str = "event_config.json";
const TEMPLATE_REPLACE_REGEX: &str = "<[^<>]+>";

#[derive(Clone, Serialize, Deserialize)]
pub struct Criterion
{
    #[serde(default)]
    check_value_path: String,
    #[serde(default)]
    check_value_in: Vec<String>,
    #[serde(default)]
    check_value_not_in: Vec<String>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Template
{
    #[serde(default="default_criteria")]
    criteria: Vec<Criterion>,
    #[serde(default)]
    body: String
}

fn default_criteria() -> Vec<Criterion> { Vec::<Criterion>::new() }

impl Template 
{
    pub fn new() -> Template
    {
        Template {criteria: Vec::<Criterion>::new(), body: String::new()}
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EventConfig
{
    hmac: String,
    templates: Vec<Template>,
    end_point: discord::request::model::Webhook,
    #[serde(default="default_privacy")]
    dont_message_on_private_repos: bool
}

fn default_privacy() -> bool 
{
    true
}

impl EventConfig
{
    pub fn get_token(&self) -> String
    {
        self.hmac.clone()
    }

    pub fn get_templates(&self) -> Vec<Template>
    {
        self.templates.clone()
    }

    pub fn get_end_point(&self) -> discord::request::model::Webhook
    {
        self.end_point.clone()
    }

    pub fn silent_on_private_repos(&self) -> bool
    {
        self.dont_message_on_private_repos
    }

    pub fn new() -> EventConfig
    {
        EventConfig 
        {
            hmac: "".to_string(), 
            templates: vec![Template::new()], 
            end_point: discord::request::model::Webhook::new("".to_string()), 
            dont_message_on_private_repos: true
        }
    }
}

pub trait Event
{
    fn get_token(&self) -> String;
    fn get_end_point(&self) -> discord::request::model::Webhook;
    fn load_config(&mut self);
    fn is_authentic(&self, headers: HeaderMap, body: Bytes) -> StatusCode;
    fn into_response(&self, data: HashMap<String, serde_json::Value>) -> (Option<String>, StatusCode);
}

pub fn read_config(name: &str) -> EventConfig
{
    if Path::new(CONFIG_PATH).exists()
    {
        let data = match read_file_utf8(CONFIG_PATH)
        {
            Some(d) => d,
            None =>
            {
                println!("Error reading configuration file {} no data", CONFIG_PATH);
                std::process::exit(1);
            }
        };

        let config: HashMap<String, EventConfig> = match serde_json::from_str(&data)
        {
            Ok(data) => {data},
            Err(why) => 
            {
                println!("Error reading configuration file {}\n{}", CONFIG_PATH, why);
                std::process::exit(1);
            }
        };

        match config.contains_key(name)
        {
            true => config[name].clone(),
            false => 
            {
                println!("Config for event, {}, not found in configuration file {}", name, CONFIG_PATH);
                std::process::exit(1);
            }
        }
    }
    else 
    {
        println!("Error configuration file {} does not exist", CONFIG_PATH);
        std::process::exit(1);
    }
}

pub fn expand_template(template: String, data: HashMap<String, serde_json::Value>) -> Option<String>
{

    if template == ""
    {
        return None
    }

    let parameters = Regex::new(TEMPLATE_REPLACE_REGEX).unwrap();
    
    let mut formatted = template.clone();
    
    for var in parameters.find_iter(&template)
    {
        let template = var.as_str().strip_prefix("<").unwrap().strip_suffix(">").unwrap();
        let path: Vec<&str> = template.split("/").collect();

        let value = match path.len()
        {
            0 => {var.as_str().to_string()},
            1 => {format!("{}", data[path[0]]).replace("\"", "")},
            _=> 
            { 
                let p = ["/", &path[1..path.len()].join("/")].join("");
                match data[path[0]].pointer(&p)
                {
                    Some(v) => format!("{}", v).replace("\"", ""),
                    None => var.as_str().to_string()
                }
               
            }
        };

        formatted = formatted.replace(var.as_str(), &value);
    }

    Some(formatted)
}

pub fn satisfied(criterion: Criterion, data: &HashMap<String, serde_json::Value>) -> bool
{

    if criterion.check_value_path == ""
    {
        return true
    }

    let path: Vec<&str> = criterion.check_value_path.split("/").collect();
        
    let extracted_value= match path.len()
    {
        0 => None,
        1 => 
        { 
            if data.contains_key(path[0])
            {
                Some(&data[path[0]])
            }
            else
            {
                None
            }
        },
        _ => 
        {
            let p = ["/", &path[1..path.len()].join("/")].join("");
            if data.contains_key(path[0])
            {
                data[path[0]].pointer(&p)
            }
            else
            {
                None
            }
            
        }
    };

    if extracted_value.is_none()
    {
        return false
    }

    let string_value = extracted_value.unwrap().to_string().replace("\"", "");

    if (criterion.check_value_in.is_empty() || criterion.check_value_in.contains(&string_value)) &&
        (criterion.check_value_not_in.is_empty() || !criterion.check_value_not_in.contains(&string_value))
    {
        return true
    }
    else
    {
        return false
    }
}

pub fn select_template(templates: Vec<Template>, data: HashMap<String, serde_json::Value>) -> String
{
    if templates.is_empty()
    {
        return "".to_string()
    }

    for template in templates
    {
        if template.criteria.into_iter().all(|c| satisfied(c, &data))
        {
            return template.body
        }
    }

    return String::new()

}