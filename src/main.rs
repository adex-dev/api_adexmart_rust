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
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}
