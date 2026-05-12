use serde::Serialize;

#[derive(Debug,Serialize)]
pub struct Responses {
    pub status: i8,
    pub status_label:String,
    pub message: String,
}
#[allow(dead_code)]
#[derive(Debug)]
pub enum UserStatus {
    Inactive = 0,
    Active = 1,
    Suspended = 2,
    Banned = 3,
    Deleted = 4,
}

