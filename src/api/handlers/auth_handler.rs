use axum::{
    Json,
    extract::State,
    http::{StatusCode},
    response::{AppendHeaders, IntoResponse},
};
use serde_json::json;

use crate::{application::{self, auth::jwt_services::generate_refresh_token}, domain, state::AppState};

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<domain::LoginRequest>,
) -> impl IntoResponse {
    let result = application::login_service(&state.db, payload).await;

    match result {
        Ok(response) => {
            let refresh_token = generate_refresh_token();
            let _cookie = format!(
                "refresh_token={}; HttpOnly; Path=/; Max-Age={}",
                refresh_token,
                7 * 24 * 60 * 60
            );
            (
                StatusCode::OK,
                AppendHeaders([("x-api-version", "v1"), ("x-powered-by", "Akmad Nudin")]),
                Json(json!({
                    "status": true,
                    "data": response
                })),
            )
        }
        Err(err) => {
        (
            StatusCode::UNAUTHORIZED,
            AppendHeaders([("x-api-version", "v1"), ("x-powered-by", "Akmad Nudin")]),
            Json(json!({
                "status": false,
                "message": err
            }))
        )
    }
    }
}
