use pulse::server::https::Server;
use pulse::program_version;

#[tokio::main]
async fn main() {

    let args: Vec<String> = std::env::args().collect();
    
    if args.iter().any(|x| x == "-v")
    {
        println!("Version: {}", program_version());
        std::process::exit(0);
    }

    let server = Server::new(0,0,0,0);

    server.serve().await;

}