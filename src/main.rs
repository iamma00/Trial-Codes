use axum::{routing::get, Router};
use reqwest::Error;
use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Deserialize)]
struct AaveRate {
    liquidityRate: String,
}

async fn get_aave_yield() -> Result<String, Error> {
    let api_url = "https://api.aave.com/data/markets";
    let response: Vec<AaveRate> = reqwest::get(api_url).await?.json().await?;
    let liquidity_rate_percentage = response[0].liquidityRate.parse::<f64>().unwrap() / 1e27 * 100.0;
    Ok(format!("{:.2}%", liquidity_rate_percentage))
}

async fn highest_yield_handler() -> String {
    let aave_yield = get_aave_yield().await.unwrap();
    aave_yield
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/highest-yield", get(highest_yield_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
