use serde::{Serialize, Deserialize};
use jsonwebtoken::{decode, encode, DecodingKey, Validation, Header, EncodingKey};
use jsonwebtoken::errors::ErrorKind;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims{
    pub iss:String,
    pub sub:String,
    pub roles:String,
    pub aud:String,

    pub exp:usize,
    pub iat:usize,
    pub nbf:usize,

    pub jti:String,
    #[serde(rename = "type")]
    pub r#type:String,
}
#[derive(Debug,Serialize,Deserialize)]
pub struct RefreshClaims{
    pub iss:String,
    pub sub:String,
    pub aud:String,

    pub exp:usize,
    pub iat:usize,
    pub nbf:usize,
    pub roles:String,
    pub jti:String,
    #[serde(rename = "type")]
    pub r#type:String,
}
use uuid::Uuid;
use crate::utils::shared::AppError;

pub struct TokenResponse {
    pub token: String,
    pub jti: Uuid,
    pub user_id: String,
    pub expires_in:chrono::NaiveDateTime,
}

pub fn verify_access_token(
    token:&str,
) -> Result<Claims,AppError>{
    let decoded= decode::<Claims>(
        token,
        &DecodingKey::from_secret(
            std::env::var("JWT_SECRET").unwrap().as_bytes(),
        ),
        &Validation::default()
    ).map_err(|e|AppError::UnauthorizedAuth(e.to_string()))?;
    let claims = decoded.claims;
    if claims.r#type !="access-token" {
        return Err(AppError::UnauthorizedAuth("Invalid Token Type".to_string()));
    }
    Ok(claims)
}

pub fn verify_refresh_token(
    token:&str,
)->Result<RefreshClaims,AppError> {
    let mut validation = Validation::default();

    validation.set_audience(&["pos-client"]);

    let decoded = decode::<RefreshClaims>(
        token,
        &DecodingKey::from_secret(
            std::env::var("JWT_SECRET").unwrap().as_bytes(),
        ),
        &validation,
    ).map_err(|e|{
        match *e.kind() {
            ErrorKind::ExpiredSignature =>{
                AppError::UnauthorizedAuth("Token Expired".to_string())
            }
            ErrorKind::InvalidToken =>{
                AppError::UnauthorizedAuth("Invalid Token".to_string())
            }
            ErrorKind::InvalidSignature =>{
                AppError::UnauthorizedAuth("Invalid Signature".to_string())
            }
            ErrorKind::InvalidAudience => {
                AppError::UnauthorizedAuth(
                    "Invalid Audience".to_string()
                )
            }
            _=>{
                AppError::UnauthorizedAuth("Token Error".to_string())
            }
        }
    })?;

    let claims = decoded.claims;
    if claims.r#type !="refresh-token" {
        return Err(AppError::UnauthorizedAuth("Invalid Token Type".to_string()));
    }
    Ok(claims)
}

pub fn encode_jwt<T>(
    claims:&T,
)->Result<String,String>
where T:Serialize{
    encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(
            std::env::var("JWT_SECRET")
                .unwrap().as_bytes(),
        ),
    ).map_err(|e| e.to_string())
}