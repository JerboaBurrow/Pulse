use crate::web::
{
    throttle::{IpThrottler, handle_throttle},
    github::{response::github_filter::filter_github, model::{GithubConfig, GithubStats}},
    discord::request::model::Webhook
};

use crate::stats;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tokio::sync::Mutex;

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
        token: String,
        disc: Webhook
    ) 
    -> ServerHttp
    {

        let requests: IpThrottler = IpThrottler::new
        (
            10.0, 
            5000
        );

        let throttle_state = Arc::new(Mutex::new(requests));

        let github = Arc::new(Mutex::new(GithubConfig::new(token, disc.clone(), GithubStats::new())));
        
        ServerHttp
        {
            addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(a,b,c,d)), port),
            router: Router::new()
            .route("/", post(|| async move {  }))
            .layer(middleware::from_fn_with_state(github, filter_github))
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