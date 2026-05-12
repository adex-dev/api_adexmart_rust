use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug,Error)]
pub enum AppError {
    #[error("Unauthorized error: {0}")]
    Unauthorized(i8,String,String),
    #[error("NotFound : {0}")]
    NotFound(i8,String,String),
    #[error("Validation : {0}")]
    Validation(i8,String,String),
    #[error("Conflict : {0}")]
    Conflict(i8,String,String),
    #[error("internal : {0}")]
    Internal(i8,String,String),
    #[error("UnauthorizedAuth : {0}")]
    UnauthorizedAuth(i8,String,String),
    #[error("BadRequest : {0}")]
    BadRequest(i8,String,String),
    #[error("Forbidden : {0}")]
    Forbidden(i8,String,String),
    #[error("Unprocessable : {0}")]
    Unprocessable(i8,String,String),
    #[error("PayloadLarge : {0}")]
    PayloadLarge(i8,String,String),
    #[error("UnsupportedMedia : {0}")]
    UnsupportedMedia(i8,String,String),
    #[error("TooManyRequest : {0}")]
    TooManyRequest(i8,String,String),
    #[error("DatabaseDown : {0}")]
    DatabaseDown(i8,String,String),
    #[error("GatewayDown : {0}")]
    GatewayDown(i8,String,String),
}
#[allow(dead_code)]
#[derive(Debug)]
pub enum AppSuccess {
    OK(i8,String,String),
    Created(i8,String,String),
    NoContent(i8,String,String),
}
