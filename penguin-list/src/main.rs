// use sqlx::Connection;
// use sqlx::Row;
use dotenv::dotenv;
use std::{env, net::SocketAddr};
// use reqwest::header::{HeaderMap, AUTHORIZATION};
use serde::Deserialize;
use axum::{
    routing::{get},
    Router,
};

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     dotenv().ok();
//     let api_key = env::var("SUPABASE_KEY")?;
//     let api_token = env::var("SUPABASE_TOKEN")?;

//     let mut headers = HeaderMap::new();
//     headers.insert(AUTHORIZATION, format!("Bearer {}", api_token).parse().unwrap());
//     headers.insert("apikey", api_key.parse().unwrap());

//     let client = reqwest::Client::new();
//     let resp = client.get("https://wrjnmpdzxbgicxhxaecq.supabase.co/rest/v1/penguins?select=*")
//         .headers(headers)
//         .send()
//         .await?;
//     println!("{:#?}", resp);
    
//     let resp_json:Vec<Penguin>= resp.json::<Vec<Penguin>>().await?;
//     println!("Json resp: {:#?}", resp_json);
//     Ok(())
// }

#[tokio::main]
async fn  main() {
    let app = Router::new()
        .route("/", get(root));
    let addr = SocketAddr::from(([0, 0 , 0, 0], 8000));
    axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}

async fn root() -> String {
    "Hello World!".to_string()
}