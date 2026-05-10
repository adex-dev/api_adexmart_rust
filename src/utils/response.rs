use serde::Serialize;

#[derive(Debug,Serialize)]
pub struct Responses {
    pub status: bool,
    pub message: String,
}