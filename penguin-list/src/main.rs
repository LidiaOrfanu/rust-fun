// use sqlx::Connection;
// use sqlx::Row;
use dotenv::dotenv;
use std::env;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use serde::Deserialize;
#[derive(Debug, Deserialize)]
struct Penguin {
    id: i32,
    name: String,
    species: String,
    age: i32
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("SUPABASE_KEY")?;
    let api_token = env::var("SUPABASE_TOKEN")?;

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {}", api_token).parse().unwrap());
    headers.insert("apikey", api_key.parse().unwrap());

    let client = reqwest::Client::new();
    let resp = client.get("https://wrjnmpdzxbgicxhxaecq.supabase.co/rest/v1/penguins?select=*")
        .headers(headers)
        .send()
        .await?;
    println!("{:#?}", resp);
    
    let resp_json:Vec<Penguin>= resp.json::<Vec<Penguin>>().await?;
    println!("Json resp: {:#?}", resp_json);
    Ok(())
}
