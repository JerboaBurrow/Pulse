pub mod web;
pub mod server;

pub mod util;

#[cfg(feature = "http")]
pub mod server_http;

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