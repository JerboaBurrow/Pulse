
use crate::web::throttle::{IpThrottler, handle_throttle};
use crate::web::response::util::{reflect, stdout_log};
use crate::web::response::github_verify::github_verify;

use std::clone;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use axum::extract::State;
use axum::
{
    routing::post, 
    Router, 
    middleware
};
use axum_server::tls_rustls::RustlsConfig;

pub struct Server
{
    addr: SocketAddr,
    router: Router
}

impl Server 
{
    pub fn new 
    (
        a: u8,
        b: u8,
        c: u8,
        d: u8,
        port: u16,
        token: String
    ) 
    -> Server
    {

        let requests: IpThrottler = IpThrottler::new
        (
            10.0, 
            5000
        );

        let throttle_state = Arc::new(Mutex::new(requests));

        let authenticated_state = token;
        

        Server
        {
            addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(a,b,c,d)), port),
            router: Router::new()
            .route("/", post(|| async move {  }))
            .layer(middleware::from_fn_with_state(authenticated_state.clone(), github_verify))
            .layer(middleware::from_fn_with_state(throttle_state.clone(), handle_throttle))

        }
    }

    pub fn get_addr(self: Server) -> SocketAddr
    {
        self.addr
    }

    pub async fn serve(self: Server, cert_path: String, key_path: String)
    {

        // configure https

        let config = match RustlsConfig::from_pem_file(
            PathBuf::from(cert_path.clone()),
            PathBuf::from(key_path.clone())
        )
        .await
        {
            Ok(c) => c,
            Err(e) => 
            {
                println!("error while reading certificates in {} and key {}\n{}", cert_path, key_path, e);
                std::process::exit(1);
            }
        };

        axum_server::bind_rustls(self.addr, config)
        .serve(self.router.into_make_service())
        .await
        .unwrap();
    }

}

pub async fn serve(token: String) {

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

    let cert_path = if args.iter().any(|x| x == "-c")
    {
        let i = args.iter().position(|x| x == "-c").unwrap();
        if i+1 < args.len()
        {
            args[i+1].clone()
        }
        else 
        {
            "./cert.pem".to_string()
        }
    }
    else
    {
        "./cert.pem".to_string()
    };

    let key_path = if args.iter().any(|x| x == "-k")
    {
        let i = args.iter().position(|x| x == "-k").unwrap();
        if i+1 < args.len()
        {
            args[i+1].clone()
        }
        else 
        {
            "./key.pem".to_string()
        }
    }
    else
    {
        "./key.pem".to_string()
    };

    let server = Server::new(0,0,0,0,port,token);

    server.serve(cert_path, key_path).await

}