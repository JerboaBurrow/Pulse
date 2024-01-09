use pulse::server::https::Server;

#[tokio::main]
async fn main() {

    let server = Server::new(0,0,0,0);

    server.serve().await;

}