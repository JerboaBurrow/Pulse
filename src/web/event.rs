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
pub struct EventConfig
{
    hmac: String,
    template: String,
    end_point: discord::request::model::Webhook
}

impl EventConfig
{
    pub fn get_token(&self) -> String
    {
        self.hmac.clone()
    }

    pub fn get_template(&self) -> String
    {
        self.template.clone()
    }

    pub fn get_end_point(&self) -> discord::request::model::Webhook
    {
        self.end_point.clone()
    }

    pub fn new() -> EventConfig
    {
        EventConfig {hmac: "".to_string(), template: "".to_string(), end_point: discord::request::model::Webhook::new("".to_string())}
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

pub fn format(template: String, data: HashMap<String, serde_json::Value>) -> String
{
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

    formatted
}