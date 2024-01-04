use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let client = reqwest::Client::new();

    let mut map = HashMap::new();
    map.insert("content", "rust-reqwest");
    
    let resp = client.post("https://discord.com/api/webhooks/1191477585914376192/_inbon-FCjm3eek7Fh-9jrCx6z_EjVRVdWj_etqJybSRkf_iQWQJBpcyeXIbEygcYImz")
        .json(&map)
        .send()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}