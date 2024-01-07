pub mod web;
pub mod server;
pub mod stats;
pub mod util;

#[cfg(feature = "http")]
pub mod server_http;

const DEBUG: bool = true;

/// Completely drop Github POST requests concerning private repos
pub const IGNORE_PRIVATE_REPOS: bool = true;

/// Process Github POST requests concerning private repos
/// but never send outbound trafic (e.g. Discord)
pub const DONT_MESSAGE_ON_PRIVATE_REPOS: bool = true;

pub fn debug(msg: String, context: Option<String>)
{
    if DEBUG == false { return }

    match context
    {
        Some(s) => println!("[DEBUG] {msg} in context {s}"),
        None => println!("[DEBUG] {msg}")
    }
    
}