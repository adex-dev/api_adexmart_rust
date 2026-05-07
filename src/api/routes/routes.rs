use axum::{
    routing::post,
    Router
};

use crate::{api::handlers::login, state::AppState};

pub fn create_routes()->Router<AppState>{
    let api_v1 = Router::new()
    .route("/login",post(login));

    Router::new().nest("/api/v1", api_v1)


}