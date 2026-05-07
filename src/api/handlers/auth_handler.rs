use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::{domain, state::AppState};

pub async fn login(
    State(state):State<AppState>,
    Json(payload): Json<domain::user::Login>
) ->impl IntoResponse {
     let hello = "Hello World".to_string();

    (
        StatusCode::OK,
        Json(json!({
            "messages": hello
        }))
    )
}