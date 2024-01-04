mod throttle;

use axum::ServiceExt;
use reqwest::get;
use throttle::throttle::{IpThrottler, handle_throttle};

use axum::http::{StatusCode, HeaderMap};
use axum::response::IntoResponse;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use axum::
{
    routing::post, 
    Router, 
    http::Request,
    middleware::{self, Next},
    response::{Response, Json},
    body::{Body, Bytes},
};

use axum_server::tls_rustls::RustlsConfig;

use openssl::sha::sha256;

use serde_json::json;

#[tokio::main]
async fn main() {

    let args: Vec<String> = std::env::args().collect();

    let port = if args.iter().any(|x| x == "-p")
    {
        let i = args.iter().position(|x| x == "-p").unwrap();
        if i+1 < args.len()
        {
            args[i+1].parse::<u16>().unwrap()
        }
        else 
        {
            3030
        }
    }
    else
    {
        3030
    };

    let ip = if args.iter().any(|x| x == "-t")
    {
        Ipv4Addr::new(127,0,0,1)
    }
    else
    {
        Ipv4Addr::new(0,0,0,0)
    };

    let requests: IpThrottler = IpThrottler::new
    (
        10.0, 
        5000
    );

    // wrap the hashmap into a Arc object, which may be cloned later (with all clones referencing the origin data)
    //  further wrap in a mutex so each thread can lock it
    //  when locked the mutex will release when going out of scope
    let state = Arc::new(Mutex::new(requests));

    // configure https

    let config = RustlsConfig::from_pem_file(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("certs")
            .join("cert.pem"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("certs")
            .join("key.pem"),
    )
    .await
    .unwrap();

    let app = Router::new()
    .route("/", post(|| async move {  }))
    .layer(middleware::from_fn(reflect));

    let addr = SocketAddr::new(IpAddr::V4(ip), port);

    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();

    async fn reflect<B>
    (
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

        let msg = std::str::from_utf8(&bytes).unwrap().to_string();
        
        println!("You sent:\n{}",msg);
        Ok(format!("You sent:\n{}",msg).into_response())
        
    }

}