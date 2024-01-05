#[cfg(feature = "http")]
use pulse::server_http::serve;

#[cfg(feature = "http")]
#[tokio::main]
async fn main() {

    let args: Vec<String> = std::env::args().collect();

    let token = if args.iter().any(|x| x == "-t")
    {
        let i = args.iter().position(|x| x == "-t").unwrap();

        if i+1 < args.len()
        {
            args[i+1].clone()
        }
        else
        {
            println!("Authentication token not provided, please provide -t token");
            std::process::exit(1);
        }
    }
    else
    {
        println!("Authentication token not provided, please provide -t token");
        std::process::exit(1);
    };

    serve(token).await;
    
}