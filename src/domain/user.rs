use chrono::NaiveDateTime;
use serde::{Deserialize,Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct Users {
    pub id : i32,
    pub id_user:String,
    pub firstname:String,
    pub lastname: String,
    pub fullname: String,
    pub username:String,
    pub roles:String,
    pub status:String,
    pub created_at:Option<NaiveDateTime>,
    pub modified:Option<NaiveDateTime>,
    pub store_id:String,
}

#[derive(Debug,Deserialize)]
pub struct Login{
    pub username :String,
    pub password :String,
}