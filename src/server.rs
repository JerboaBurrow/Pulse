
use crate::web::throttle::{IpThrottler, handle_throttle};
use crate::web::response::util::{reflect, stdout_log};

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::{Arc, Mutex};

use axum::
{
    routing::post, 
    Router, 
    middleware
};

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
        port: u16
    ) 
    -> Server
    {

        let requests: IpThrottler = IpThrottler::new
        (
            10.0, 
            5000
        );

        let state = Arc::new(Mutex::new(requests));

        Server
        {
            addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(a,b,c,d)), port),
            router: Router::new()
            .route("/", post(|| async move {  }))
            .layer(middleware::from_fn(stdout_log))
            .layer(middleware::from_fn_with_state(state.clone(), handle_throttle))
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

pub async fn serve() {

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

    let server = if args.iter().any(|x| x == "-t")
    {
        Server::new(127,0,0,1,port)
    }
    else
    {
        Server::new(0,0,0,0,port)
    };

    server.serve().await

}