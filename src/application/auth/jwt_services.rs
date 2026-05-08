use chrono::{
    Utc,
    Duration,
};
use uuid::Uuid;

use jsonwebtoken::{
    encode,
    EncodingKey,
    Header,
};

use crate::utils::jwt::Claims;

pub fn generate_access_token(
    user_id:&str,
    roles:&str,
)->Result<String,String>{
    let expiration = Utc::now() + Duration::minutes(15);
    let claims = Claims{
        sub:user_id.to_string(),
        roles:roles.to_string(),
        exp:expiration.timestamp() as usize,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(
            b"supersecretkeyakmadnudin"
        ),
    ).map_err(|_| "Generate token gagal")?;

    Ok(token)
}

pub fn generate_refresh_token()->String{
    Uuid::new_v4().to_string()
}