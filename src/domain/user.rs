use chrono::NaiveDateTime;
use serde::{Deserialize,Serialize};
use sqlx::FromRow;

use crate::domain::store_entity::LoginStore;


#[derive(Debug,FromRow)]
pub struct UserEntity {
    pub id : i32,
    pub id_user:String,
    pub firstname:String,
    pub lastname: String,
    pub fullname: String,
    pub username:String,
    pub password:String,
    pub roles:String,
    pub status:String,
    pub created_at:Option<NaiveDateTime>,
    pub modifed_at:Option<NaiveDateTime>,
    pub store_id:String,
}
#[derive(Debug,Serialize)]
pub struct UserResponse {
    pub id : i32,
    pub id_user:String,
    pub firstname:String,
    pub lastname: String,
    pub fullname: String,
    pub username:String,
    pub roles:String,
    pub status:String,
    pub created_at:Option<NaiveDateTime>,
    pub modifed_at:Option<NaiveDateTime>,
    pub store_id:String,
}

#[derive(Debug,Deserialize)]
pub struct LoginRequest{
    pub username :String,
    pub password :String,
}

#[derive(Debug,Serialize)]
pub struct LoginResult {
    pub refresh_token: TokenEntity,
    pub response_data: LoginResponse,
}
#[derive(Debug,Serialize)]
pub struct TokenEntity {
    pub token: String,
}
#[derive(Debug,Serialize)]
pub struct  LoginResponse{
    pub user:UserResponse,
    pub access_token:String,
    pub modul_access: Vec<UserAccessResponse>,
    pub store:Vec<LoginStore>
}


#[derive(Debug,Serialize)]
pub struct UserAccessResponse{
    pub id:i32,
    pub  name:String
}

#[derive(FromRow)]
pub struct UserAccessEntity{
    pub id:i32,
    pub  name:String
}

#[derive(Debug,FromRow,Serialize)]
pub struct TokenRefreshEntity{
    pub token_hash:String,
    pub expires_at:chrono::NaiveDateTime,
    pub revoked:bool
}