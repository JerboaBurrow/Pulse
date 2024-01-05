
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

pub struct ServerHttp
{
    addr: SocketAddr,
    router: Router
}

impl ServerHttp
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
    -> ServerHttp
    {

        let requests: IpThrottler = IpThrottler::new
        (
            10.0, 
            5000
        );

        let throttle_state = Arc::new(Mutex::new(requests));

        let authenticated_state = token;
        

        ServerHttp
        {
            addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(a,b,c,d)), port),
            router: Router::new()
            .route("/", post(|| async move {  }))
            .layer(middleware::from_fn_with_state(authenticated_state.clone(), github_verify))
            .layer(middleware::from_fn_with_state(throttle_state.clone(), handle_throttle))

        }
    }

    pub fn get_addr(self: ServerHttp) -> SocketAddr
    {
        self.addr
    }

    pub async fn serve(self: ServerHttp)
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

    let server = ServerHttp::new(127,0,0,1,port,token);

    server.serve().await

}