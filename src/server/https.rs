use crate::web::
{
    throttle::{IpThrottler, handle_throttle},
    github::{response::github_filter::filter_github, model::GithubStats},
    discord::request::model::Webhook
};

use tokio::task::spawn;

use crate::stats;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

use axum::
{
    routing::{post, get}, 
    Router, 
    middleware, response::Redirect
};
use axum_server::tls_rustls::RustlsConfig;

use super::model::AppState;

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
        disc: Webhook,
    ) 
    -> Server
    {

        let requests: IpThrottler = IpThrottler::new
        (
            10.0, 
            5000
        );

        let throttle_state = Arc::new(Mutex::new(requests));

        let github = Arc::new(Mutex::new(AppState::new(GithubStats::new())));

        let _stats_watcher = spawn(stats::io::watch(Webhook::new(disc.get_addr()), github.clone()));

        Server
        {
            addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(a,b,c,d)), port),
            router: Router::new()
            .route("/", post(|| async move {  }))
            .layer(middleware::from_fn_with_state(github, filter_github))
            .layer(middleware::from_fn_with_state(throttle_state.clone(), handle_throttle))
            .route("/badge", get(|| async move {Redirect::permanent("https://badgen.net/badge/Pulse/live/green?icon=discord")}))
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
        .serve(self.router.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
    }

}