use chrono::{
    Utc,
    Duration,
};
use uuid::Uuid;

use crate::utils::jwt::{encode_jwt, Claims, RefreshClaims, TokenResponse};

pub fn generate_access_token(
    user_id:&str,
    roles:&str,
)->Result<String,String>{
    let now = Utc::now();
    let expiration = now + Duration::minutes(15);
    let jti = Uuid::new_v7();
    let user_id =user_id.to_string();
    let claims = Claims{
        iss:"pos-system".to_string(),
        sub:user_id.clone(),
        roles:roles.to_string(),
        aud:"pos-client".to_string(),
        exp:expiration.timestamp() as usize,
        iat: now.timestamp() as usize,
        nbf:now.timestamp() as usize,
        jti:jti.clone().to_string(),
        r#type:"access-token".to_string(),
    };
    let token = encode_jwt(&claims)?;
    Ok(token)
}

pub fn generate_refresh_token(
    user_id:&str,
    roles:&str,
)->Result<TokenResponse,String>{
    let now = Utc::now();
    let expiration = now + Duration::days(7);
    let jti = Uuid::new_v7();
    let claims = RefreshClaims{
        iss:"pos-system".to_string(),
        sub:user_id.to_string(),
        aud:"pos-client".to_string(),
        exp:expiration.timestamp() as usize,
        roles:roles.to_string(),
        iat: now.timestamp() as usize,
        nbf:now.timestamp() as usize,
        jti:jti.clone().to_string(),
        r#type:"refresh-token".to_string(),
    };
    let token = encode_jwt(&claims)?;
    Ok(TokenResponse {
        token,
        jti,
        user_id:user_id.to_string(),
        expires_in:expiration.naive_utc(),
    })
}

