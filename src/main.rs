use pulse::{server::https::Server, web::discord::request::model::Webhook};

#[tokio::main]
async fn main() {

    let args: Vec<String> = std::env::args().collect();

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

    let cert_path = if args.iter().any(|x| x == "-c")
    {
        let i = args.iter().position(|x| x == "-c").unwrap();
        if i+1 < args.len()
        {
            args[i+1].clone()
        }
        else 
        {
            "./cert.pem".to_string()
        }
    }
    else
    {
        "./cert.pem".to_string()
    };

    let key_path = if args.iter().any(|x| x == "-k")
    {
        let i = args.iter().position(|x| x == "-k").unwrap();
        if i+1 < args.len()
        {
            args[i+1].clone()
        }
        else 
        {
            "./key.pem".to_string()
        }
    }
    else
    {
        "./key.pem".to_string()
    };

    let server = Server::new(0,0,0,0, port, Webhook::new(disc_url));

    server.serve(cert_path, key_path).await;

}