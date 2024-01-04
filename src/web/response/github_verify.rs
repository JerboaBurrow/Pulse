//! Utility responses for the axum server

use axum::extract::State;
use axum::http::{StatusCode, HeaderMap};
use axum::response::IntoResponse;

use axum::
{
    http::Request,
    middleware::Next,
    response::Response,
    body::Bytes,
};

use chrono::Local;
use openssl::hash::MessageDigest;
use openssl::memcmp;
use openssl::pkey::PKey;
use openssl::sign::Signer;
use regex::Regex;

use std::fmt::Write;
use std::sync::{Arc, Mutex};

pub fn dump_bytes(v: &[u8]) -> String 
{
    let mut byte_string = String::new();
    for &byte in v
    {
        write!(&mut byte_string, "{:0>2X}", byte).expect("byte dump error");
    };
    byte_string
}

pub fn read_bytes(v: String) -> Vec<u8>
{
    (0..v.len()).step_by(2)
    .map
    (
        |index| u8::from_str_radix(&v[index..index+2], 16).unwrap()
    )
    .collect()
}

pub async fn github_verify<B>
(
    State(app_state): State<Arc<Mutex<crate::server::Config>>>,
    headers: HeaderMap,
    request: Request<B>,
    next: Next<B>
) -> Result<Response, StatusCode>
where B: axum::body::HttpBody<Data = Bytes>
{
    let (_parts, body) = request.into_parts();

    let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(_) => {
            return Err(StatusCode::BAD_REQUEST)
        }
    };

    match headers.contains_key("x-hub-signature-256")
    {
        false => 
        {
            crate::debug("no signature".to_string(), None);
            return Ok((StatusCode::UNAUTHORIZED).into_response())
        },
        true => {}
    };

    let signature = match std::str::from_utf8(headers["x-hub-signature-256"].as_bytes())
    {
        Ok(s) => s,
        Err(_) => 
        {
            crate::debug("signature utf8 parse failure".to_string(), None);
            return Ok((StatusCode::BAD_REQUEST).into_response())
        }
    };

    let post_digest = Regex::new(r"sha256=").unwrap().replace_all(signature, "").into_owned().to_uppercase();

    let token = app_state.lock().unwrap().token.clone();
    let key = match PKey::hmac(token.as_bytes())
    {
        Ok(k) => k,
        Err(_) => 
        {
            crate::debug("key creation failure".to_string(), None);
            return Ok((StatusCode::INTERNAL_SERVER_ERROR).into_response())
        }
    };

    let mut signer = match Signer::new(MessageDigest::sha256(), &key)
    {
        Ok(k) => k,
        Err(_) => 
        {
            crate::debug("signer creation failure".to_string(), None);
            return Ok((StatusCode::INTERNAL_SERVER_ERROR).into_response())
        }
    };
    
    match signer.update(&bytes)
    {
        Ok(k) => k,
        Err(_) => 
        {
            crate::debug("signing update failure".to_string(), None);
            return Ok((StatusCode::INTERNAL_SERVER_ERROR).into_response())
        }
    };

    let hmac = match signer.sign_to_vec()
    {
        Ok(k) => k,
        Err(_) => 
        {
            crate::debug("sign failure".to_string(), None);
            return Ok((StatusCode::INTERNAL_SERVER_ERROR).into_response())
        }
    };

    crate::debug(format!("post_digtest: {}, len: {}\nlocal hmac: {}, len: {}", post_digest, post_digest.len(), dump_bytes(&hmac), dump_bytes(&hmac).len()), None);

    match memcmp::eq(&hmac, &read_bytes(post_digest.clone()))
    {
        true => {},
        false => 
        {
            crate::debug(format!("bad signature: local/post\n{}\n{}", post_digest, dump_bytes(&hmac)), None);
            return Ok((StatusCode::UNAUTHORIZED).into_response())
        }
    }

    // it is now safe to process the POST request

    let body = std::str::from_utf8(&bytes).unwrap().to_string();

    crate::debug(format!("[{}] Got request:\n\nheader:\n\n{:?}\n\nbody:\n\n{}", Local::now(), headers, body), None);
    
    Ok((StatusCode::OK).into_response())


    
}