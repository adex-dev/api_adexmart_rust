use chrono::Local;
use regex::Regex;
use crate::utils::Responses;
use axum::{http::StatusCode, response::{AppendHeaders, IntoResponse, Response}, Json};
use serde_json::json;
use crate::utils::shared::{AppError, AppSuccess};

//format : 2026-05-07 14:30:00
#[allow(dead_code)]
pub fn time_now_modif()->String{
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

// trim + lower
#[allow(dead_code)]
pub fn convert_to_normalize_lower(value:&str)->String{
    value.trim().to_lowercase()
}

// check empty string

pub fn is_empty(value:&str)->bool{
    value.trim().is_empty()
}


/// Check not empty string
#[allow(dead_code)]
pub fn not_empty(value: &str) -> bool {
    !value.trim().is_empty()
}

/// Valid name:
/// a-z A-Z 0-9 . space
#[allow(dead_code)]
pub fn is_valid_name(name: &str) -> bool {
    let regex = Regex::new(r"^[a-zA-Z0-9.\s]+$").unwrap();
    regex.is_match(name)
}
#[allow(dead_code)]
/// Validate email
pub fn is_valid_email(email: &str) -> bool {
    let regex =
        Regex::new(r"^[a-zA-Z0-9._%+\-]+@[a-zA-Z0-9.\-]+\.[a-zA-Z]{2,}$").unwrap();

    regex.is_match(email)
}

/// Validate price
/// only number + dot + space
#[allow(dead_code)]
pub fn is_valid_price(price: &str) -> bool {
    let regex = Regex::new(r"^[0-9.\s]+$").unwrap();
    regex.is_match(price)
}

/// Example:
/// 1 => 000001
#[allow(dead_code)]
pub fn sequence_number(n: i32) -> String {
    format!("{:06}", n)
}

/// Example:
/// INV-2605000001
#[allow(dead_code)]
pub fn prefix_module(code: &str, seq: i32) -> String {
    let date = Local::now().format("%y%m").to_string();

    format!("{}-{}{}", code, date, sequence_number(seq))
}


pub fn splits_prefix_module(code:&str, value:&str) ->bool{
    value.trim().to_lowercase().starts_with(&format!("{}-",code.trim().to_lowercase()))
}

pub fn response_body(
    status_code: StatusCode,
    status: &bool,
    message: &str,
) -> Response {

    (
        status_code,
        AppendHeaders([
            ("x-api-version", "v1"),
            ("x-powered-by", "Akmad Nudin"),
        ]),
        Json(Responses {
            status: *status,
            message: message.to_string(),
        }),
    ).into_response()
}
impl IntoResponse for AppError {
    fn into_response(self) -> Response {

        let (status_code, message) = match self {

            AppError::Unauthorized(msg) => {
                (StatusCode::UNAUTHORIZED, msg)
            }
            AppError::UnauthorizedAuth(msg) => {
                return  auth_error(&msg);
            }
            AppError::NotFound(msg) => {
                (StatusCode::NOT_FOUND, msg)
            }
            AppError::Validation(msg) => {
                (StatusCode::UNPROCESSABLE_ENTITY, msg)
            }
            AppError::Conflict(msg) => {
                (StatusCode::CONFLICT, msg)
            }
            AppError::Internal(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
            AppError::BadRequest(msg) => {
                (StatusCode::BAD_REQUEST, msg)
            }
            AppError::Forbidden(msg) => {
                (StatusCode::FORBIDDEN, msg)
            }
            AppError::Unprocessable(msg) => {
                (StatusCode::UNPROCESSABLE_ENTITY, msg)
            }
            AppError::PayloadLarge(msg) => {
                (StatusCode::PAYLOAD_TOO_LARGE, msg)
            }
            AppError::UnsupportedMedia(msg) => {
                (StatusCode::UNSUPPORTED_MEDIA_TYPE, msg)
            }
            AppError::TooManyRequest(msg) => {
                (StatusCode::TOO_MANY_REQUESTS, msg)
            }
            AppError::DatabaseDown(msg) => {
                (StatusCode::SERVICE_UNAVAILABLE, msg)
            }
            AppError::GatewayDown(msg) => {
                (StatusCode::GATEWAY_TIMEOUT, msg)
            }
        };

        (
            status_code,
            response_body(status_code,&false, &message),
        )
            .into_response()
    }
}
#[allow(dead_code)]
pub fn success_body(
    status: StatusCode,
    message: &str,
) -> Response {

    (
        status,
        AppendHeaders([
            ("x-api-version", "v1"),
            ("x-powered-by", "Akmad Nudin"),
        ]),
        Json(Responses {
            status: true,
            message: message.to_string(),
        }),
    ).into_response()
}

impl IntoResponse for AppSuccess{
    fn into_response(self) -> Response {
        let (status_code,messsage) = match self {
            AppSuccess::OK(msg) => {
                (StatusCode::OK,msg)
            }

            AppSuccess::Created(msg) =>{
                (StatusCode::CREATED, msg)
            }
            AppSuccess::NoContent(msg) =>{
                (StatusCode::NO_CONTENT, msg)
            }

        };
        (
            status_code,
            response_body(status_code,&true, &messsage),
            ).into_response()
    }
}

pub fn auth_error(
    message: &str,
) -> Response {

    (
        StatusCode::UNAUTHORIZED,
        AppendHeaders([
            (
                "set-cookie",
                "refresh_token=; HttpOnly; Path=/; Max-Age=0",
            ),
            ("x-api-version", "v1"),
            ("x-powered-by", "Akmad Nudin"),
        ]),
        Json(json!({
            "status": false,
            "message": message
        })),
    )
        .into_response()
}

pub  fn required_access_roles() -> Vec<String> {
    std::env::var("REQUIRED_ACCESS")
        .unwrap_or_default()
        .split(',')
        .map(|s| s.trim().to_string()).collect()
}
pub  fn required_access_store_roles() -> Vec<String> {
    std::env::var("REQUIRED_ACCESS_STORE")
        .unwrap_or_default()
        .split(',')
        .map(|s| s.trim().to_string()).collect()
}