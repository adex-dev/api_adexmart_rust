use chrono::NaiveDateTime;
use serde::{Serialize};
use sqlx::FromRow;

#[derive(Debug,Serialize)]
pub struct  LoginStore{
    pub store_id:String,
    pub name:String,
    pub address:String,
    pub cities:String,
    pub province:String,
    pub phone1:String,
    pub phone2:String,
    pub parent_area:String,
}
#[allow(dead_code)]
#[derive(FromRow)]
pub struct  StoreEntity{
    #[sqlx(default)]
    pub id: i32,
    pub store_id:String,
    pub name:String,
    pub address:String,
    pub cities:String,
    pub province:String,
    pub phone1:String,
    pub phone2:String,
    pub status:String,
    #[sqlx(default)]
    pub parent_id:i16,
    pub parent_area:String,
    pub created_at:Option<NaiveDateTime>,
    pub modified:Option<NaiveDateTime>,
}