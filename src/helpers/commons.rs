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
    status: &i8,
    status_label: &str,
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
            status_label:status_label.to_string(),
            message: message.to_string(),
        }),
    ).into_response()
}
impl IntoResponse for AppError {
    fn into_response(self) -> Response {

        let (status_code,status,status_label, message) = match self {

            AppError::Unauthorized(status,status_label,msg) => {
                (StatusCode::UNAUTHORIZED,status,status_label, msg)
            }
            AppError::UnauthorizedAuth(status,status_label,msg) => {
                return  auth_error(&status,&status_label,&msg);
            }
            AppError::NotFound(status,status_label,msg) => {
                (StatusCode::NOT_FOUND,status,status_label, msg)
            }
            AppError::Validation(status,status_label,msg) => {
                (StatusCode::UNPROCESSABLE_ENTITY,status,status_label, msg)
            }
            AppError::Conflict(status,status_label,msg) => {
                (StatusCode::CONFLICT,status,status_label, msg)
            }
            AppError::Internal(status,status_label,msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR,status,status_label, msg)
            }
            AppError::BadRequest(status,status_label,msg) => {
                (StatusCode::BAD_REQUEST,status,status_label, msg)
            }
            AppError::Forbidden(status,status_label,msg) => {
                (StatusCode::FORBIDDEN,status,status_label, msg)
            }
            AppError::Unprocessable(status,status_label,msg) => {
                (StatusCode::UNPROCESSABLE_ENTITY,status,status_label, msg)
            }
            AppError::PayloadLarge(status,status_label,msg) => {
                (StatusCode::PAYLOAD_TOO_LARGE,status,status_label, msg)
            }
            AppError::UnsupportedMedia(status,status_label,msg) => {
                (StatusCode::UNSUPPORTED_MEDIA_TYPE,status,status_label, msg)
            }
            AppError::TooManyRequest(status,status_label,msg) => {
                (StatusCode::TOO_MANY_REQUESTS,status,status_label, msg)
            }
            AppError::DatabaseDown(status,status_label,msg) => {
                (StatusCode::SERVICE_UNAVAILABLE,status,status_label, msg)
            }
            AppError::GatewayDown(status,status_label,msg) => {
                (StatusCode::GATEWAY_TIMEOUT,status,status_label, msg)
            }
        };

        (
            status_code,
            response_body(status_code,&status,&status_label, &message),
        )
            .into_response()
    }
}
#[allow(dead_code)]
pub fn success_body(
    status_code: StatusCode,
    status_label: &str,
    status:&i8,
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
            status_label:status_label.to_string(),
            message: message.to_string(),
        }),
    ).into_response()
}

impl IntoResponse for AppSuccess{
    fn into_response(self) -> Response {
        let (status_code,status,status_label,messsage) = match self {
            AppSuccess::OK(status,status_label,msg) => {
                (StatusCode::OK,status,status_label,msg)
            }

            AppSuccess::Created(status,status_label,msg) =>{
                (StatusCode::CREATED,status,status_label, msg)
            }
            AppSuccess::NoContent(status,status_label,msg) =>{
                (StatusCode::NO_CONTENT,status,status_label, msg)
            }

        };
        (
            status_code,
            response_body(status_code,&status,&status_label, &messsage),
            ).into_response()
    }
}

pub fn auth_error(
    status:&i8,
    status_label:&str,
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
            "status": *status,
            "status_label":status_label,
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