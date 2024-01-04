
use crate::web::throttle::{IpThrottler, handle_throttle};
use crate::web::response::util::{reflect, stdout_log};
use crate::web::response::github_verify::github_verify;

use std::clone;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::{Arc, Mutex};

use axum::extract::State;
use axum::
{
    routing::post, 
    Router, 
    middleware
};

pub struct Config
{
    pub throttle: IpThrottler,
    pub token: String
}

impl Config
{
    pub fn new(max_requests_per_second: f64, timeout_millis: u128, t: String) -> Config
    {
        Config 
        {
            throttle: IpThrottler::new(max_requests_per_second, timeout_millis),
            token: t
        }
    }
}

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

        let config = Config::new(10.0, 5000, token);
        let app_state = Arc::new(Mutex::new(config));

        Server
        {
            addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(a,b,c,d)), port),
            router: Router::new()
            .route("/", post(|| async move {  }))
            .layer(middleware::from_fn_with_state(app_state.clone(), github_verify))
            .layer(middleware::from_fn_with_state(app_state.clone(), handle_throttle))

        }
    }

    pub fn get_addr(self: Server) -> SocketAddr
    {
        self.addr
    }

    pub async fn serve(self: Server)
    {
        axum::Server::bind(&self.addr)
        .serve(self.router.into_make_service_with_connect_info::<SocketAddr>())
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

    let server = Server::new(127,0,0,1,port,token);

    server.serve().await

}