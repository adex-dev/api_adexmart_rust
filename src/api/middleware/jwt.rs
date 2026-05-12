use axum::{
    extract::Request,
    http::{header},
    middleware::Next,
    response::Response,
};
use crate::application::auth::crypto::{api_token_decrypt};
use crate::utils::jwt::{verify_access_token};
use crate::utils::shared::AppError;

pub async fn jwt_middleware(
    mut req: Request,
    next: Next,
)->Result<Response,AppError>{
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    let auth_header = match auth_header {
        Some(v)=>v,
        None => {
            return  Err(AppError::UnauthorizedAuth(0i8,"Authorization".to_string(),"Tidak dapat akses perlu token".to_string()));
        }
    };
    let token = match auth_header.strip_prefix("Bearer ") {
        Some(v)=>v,
        None => {
            return  Err(AppError::UnauthorizedAuth(0i8,"Authorization".to_string(),"Format token salah".to_string()));
        }
    };
    let decrypt_token = api_token_decrypt(token)?;
    let claims = match verify_access_token(&decrypt_token) {
        Ok(v)=>v,
        Err(_) => {
            return  Err(AppError::UnauthorizedAuth(0i8,"Authorization".to_string(),"Token tidak Valid".to_string()));
        }
    };
    req.extensions_mut().insert(claims);
    Ok(next.run(req).await)
}