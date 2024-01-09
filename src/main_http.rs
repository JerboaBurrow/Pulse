#[cfg(feature = "http")]

use pulse::{server::http::ServerHttp, web::discord::request::model::Webhook, stats};

#[cfg(feature = "http")]
#[tokio::main]
async fn main() {

    let server = ServerHttp::new(0,0,0,0);

    server.serve().await;
    
}