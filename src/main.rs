mod api;
mod application;
mod config;
mod domain;
mod infrastructure;
mod state;
mod utils;
mod helpers;

use crate::state::AppState;
use axum::{Router};
use dotenvy::dotenv;
use tokio::net::TcpListener;
#[tokio::main]
async fn main() {
    dotenv().ok();
    let db = config::connect_db().await;
    let app_state = AppState {db };
    let app = Router::new().merge(api::routes::create_routes()).with_state(app_state);
    let app_run = format!(
        "{}:{}",
        std::env::var("APP_IP").unwrap(),
        std::env::var("APP_PORT").unwrap()
    );
    let listener = TcpListener::bind(app_run).await.unwrap();
    println!("Running On Port:{}",listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap()
}
