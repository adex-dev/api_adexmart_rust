use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug,Error)]
pub enum AppError {
    #[error("Unauthorized error: {0}")]
    Unauthorized(String),
    #[error("NotFound : {0}")]
    NotFound(String),
    #[error("Validation : {0}")]
    Validation(String),
    #[error("Conflict : {0}")]
    Conflict(String),
    #[error("internal : {0}")]
    Internal(String),
    #[error("UnauthorizedAuth : {0}")]
    UnauthorizedAuth(String),
    #[error("BadRequest : {0}")]
    BadRequest(String),
    #[error("Forbidden : {0}")]
    Forbidden(String),
    #[error("Unprocessable : {0}")]
    Unprocessable(String),
    #[error("PayloadLarge : {0}")]
    PayloadLarge(String),
    #[error("UnsupportedMedia : {0}")]
    UnsupportedMedia(String),
    #[error("TooManyRequest : {0}")]
    TooManyRequest(String),
    #[error("DatabaseDown : {0}")]
    DatabaseDown(String),
    #[error("GatewayDown : {0}")]
    GatewayDown(String),
}
#[allow(dead_code)]
#[derive(Debug)]
pub enum AppSuccess {
    OK(String),
    Created(String),
    NoContent(String),
}
