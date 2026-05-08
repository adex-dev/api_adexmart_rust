use bcrypt::{
    // hash,
    verify,
    // DEFAULT_COST,
};

// pub fn hashed_password(password: &str)->Result<String,bcrypt::BcryptError>{
//     hash(password,DEFAULT_COST)
// }
pub fn check_password(hashed_password:&str,password:&str)->Result<bool,bcrypt::BcryptError>{
    verify(password,hashed_password)
}