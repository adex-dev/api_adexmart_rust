use axum::{
    routing::post,
    Router
};

use crate::{api::handlers::login, state::AppState};

pub fn create_routes()->Router<AppState>{
     let logi  = Router::new()
    .route("/login",post(login));
    let api_v2 = Router::new()
    .route("/login",post(login));

    Router::new().nest("/api", logi).nest("/api/v1", api_v2)


}