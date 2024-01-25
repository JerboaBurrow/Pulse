#[cfg(feature = "http")]

use pulse::server::http::ServerHttp;
use pulse::program_version;

#[cfg(feature = "http")]
#[tokio::main]
async fn main() {

    let args: Vec<String> = std::env::args().collect();
    
    if args.iter().any(|x| x == "-v")
    {
        println!("Version: {}", program_version());
        std::process::exit(0);
    }

    let server = ServerHttp::new(0,0,0,0);

    server.serve().await;
    
}