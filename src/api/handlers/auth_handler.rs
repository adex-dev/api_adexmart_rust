use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{AppendHeaders, IntoResponse},
};
use axum_extra::extract::CookieJar;
use serde_json::json;

use crate::application::auth::{crypto::api_token_encrypt, jwt_services::generate_access_token};
use crate::application::user::auth_services::{check_token_refresh, check_token_refresh_full};
use crate::utils::jwt::verify_refresh_token;
use crate::utils::shared::AppError;
use crate::{application, domain, state::AppState};

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<domain::LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    let result = application::login_service(&state.db, payload).await?;

    let cookie = format!(
        "refresh-token={}; HttpOnly; Path=/; Max-Age={}",
        result.refresh_token.token.to_string(),
        7 * 24 * 60 * 60
    );

    Ok((
        StatusCode::OK,
        AppendHeaders([
            ("set-cookie", cookie),
            ("x-api-version", "v1".to_string()),
            ("x-powered-by", "Akmad Nudin".to_string()),
        ]),
        Json(json!({
            "status": true,
            "data": result.response_data,
        })),
    ))
}

pub async fn refresh(
    jar: CookieJar,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let cookie = jar.get("refresh-token");
    let token_header = cookie
        .ok_or(AppError::UnauthorizedAuth(
            "Cookie tidak ditemukan".to_string(),
        ))?
        .value();
    let claims = verify_refresh_token(token_header)?;
    let row_token_db = check_token_refresh(&state.db, &claims.jti, token_header)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    if !row_token_db {
        return Err(AppError::Internal(
            "Refresh token gagal Tidak ada".to_string(),
        ));
    }
    let access_token = generate_access_token(&claims.sub.to_string(), &claims.roles.to_string())
        .map_err(|_| AppError::UnauthorizedAuth("Generate token access gagal".to_string()))?;
    let token_encryption = api_token_encrypt(&access_token)?;
    Ok((
        StatusCode::OK,
        AppendHeaders([
            ("x-api-version", "v1".to_string()),
            ("x-powered-by", "Akmad Nudin".to_string()),
        ]),
        Json(json!({
            "status": true,
            "data": {"access_token": token_encryption},
        })),
    ))
}

pub async fn refresh_full(
    jar: CookieJar,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let cookie = jar.get("refresh-token");
    let token_header = cookie
        .ok_or(AppError::UnauthorizedAuth(
            "Cookie tidak ditemukan".to_string(),
        ))?
        .value();

    let result = check_token_refresh_full(&state.db, &token_header)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok((
        StatusCode::OK,
        AppendHeaders([
            ("x-api-version", "v1".to_string()),
            ("x-powered-by", "Akmad Nudin".to_string()),
        ]),
        Json(json!({
            "status": true,
            "data": result,
        })),
    ))
}
