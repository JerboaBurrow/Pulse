use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args: Vec<String> = std::env::args().collect();

    let msg = if args.iter().any(|x| x == "-m")
    {
        let i = args.iter().position(|x| x == "-m").unwrap();

        if i+1 < args.len()
        {
            args[i+1].clone()
        }
        else
        {
            "posted from rust!".to_string()
        }
    }
    else
    {
        "posted from rust!".to_string()
    };

    let webhook = if args.iter().any(|x| x == "-s")
    {
        let i = args.iter().position(|x| x == "-s").unwrap();

        if i+1 < args.len()
        {
            args[i+1].clone()
        }
        else
        {
            println!("No webhook given after -s");
            std::process::exit(1);
        }
    }
    else
    {
        println!("No webhook given, please provide one with -s https://discord.com/api/webhooks/xxx/yyy");
        std::process::exit(1);
    };
    
    let client = reqwest::Client::new();

    let mut map = HashMap::new();
    map.insert("content", &msg);
    
    let resp = client.post(&webhook)
        .json(&map)
        .send()
        .await?;
    println!("{:#?}", resp);
    Ok(())
    
}