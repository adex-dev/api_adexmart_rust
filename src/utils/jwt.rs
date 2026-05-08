use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct Claims{
    pub  sub:String,
    pub roles:String,
    pub exp:usize,
}