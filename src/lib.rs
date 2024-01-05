pub mod discord;

pub mod web;
pub mod server;

const DEBUG: bool = true;

pub fn debug(msg: String, context: Option<String>)
{
    if DEBUG == false { return }

    match context
    {
        Some(s) => println!("[DEBUG] {msg} in context {s}"),
        None => println!("[DEBUG] {msg}")
    }
    
}