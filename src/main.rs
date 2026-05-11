mod api;
mod application;
mod config;
mod domain;
mod infrastructure;
mod state;
mod utils;
mod helpers;

use std::net::SocketAddr;
use crate::state::AppState;
use axum::{Router};
use dotenvy::dotenv;
use tokio::net::TcpListener;
use tower_governor::governor::GovernorConfigBuilder;
use tower_governor::GovernorLayer;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db = config::connect_db().await;
    let storage_path = std::env::var("STORAGE_PATH")
        .expect("Storage Path mesti di setup");
    let app_state = AppState {db,storage_path };
    let gov_layer = GovernorConfigBuilder::default()
        .per_second(5)
        .burst_size(10)
        .finish()
        .expect("access mencapai limit");

    let app = Router::new().merge(api::routes::create_routes()).with_state(app_state).layer(GovernorLayer::new(gov_layer));
    let addr = format!(
        "{}:{}",
        std::env::var("APP_IP").unwrap(),
        std::env::var("APP_PORT").unwrap()
    );
    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let app_run: SocketAddr = addr
        .parse()
        .expect("Format address tidak valid");
    tracing::debug!("listening on {}", app_run);

    let listener = TcpListener::bind(app_run).await.unwrap();
    println!("Running On Port:{}",listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap()
}
