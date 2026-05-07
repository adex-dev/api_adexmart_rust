use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;


pub async fn login() ->impl IntoResponse {
     let hello = "Hello World".to_string();

    (
        StatusCode::OK,
        Json(json!({
            "messages": hello
        }))
    )
}