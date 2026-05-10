use base64::Engine;
use base64::engine::general_purpose;
use crate::utils::shared::AppError;

pub fn load_key() ->Result<[u8; 32],AppError>{

    let key_b64 = std::env::var("CRYPTO_KEY")
        .map_err(|e| {
            println!("{}", e);
            AppError::Internal("Missing CRYPTO_KEY environment variable.".to_string())
        })?;
    let decoded_key = general_purpose::STANDARD.decode(&key_b64)
        .map_err(|e| {
            println!("{}", e.to_string());
            AppError::Internal("Decode error.".to_string())
        })?;

    if decoded_key.len() != 32 {
        return Err(AppError::Internal(
            format!("Invalid key length: {}", decoded_key.len())
        ));
    }

    let mut key = [0u8; 32];
    key.copy_from_slice(&decoded_key);
    Ok(key)
}