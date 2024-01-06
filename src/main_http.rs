#[cfg(feature = "http")]

use pulse::{server_http::ServerHttp, web::request::discord::model::Webhook};

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

    let disc_url = if args.iter().any(|x| x == "-w")
    {
        let i = args.iter().position(|x| x == "-w").unwrap();
        if i+1 < args.len()
        {
            args[i+1].clone()
        }
        else 
        {
            println!("Discord webhook url not provided, please provide -w https://discord.com/api/webhooks/xxx/yyy");
            std::process::exit(1);
        }
    }
    else
    {
        println!("Discord webhook url not provided, please provide -w https://discord.com/api/webhooks/xxx/yyy");
        std::process::exit(1);
    };

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


    let server = ServerHttp::new(0,0,0,0, port,token, Webhook::new(disc_url));

    server.serve().await;
    
}