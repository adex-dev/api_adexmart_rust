use base64::Engine;
use base64::engine::general_purpose;
use chacha20poly1305::{aead::{Aead, KeyInit, OsRng, rand_core::RngCore}, ChaCha20Poly1305, Key, Nonce};
use crate::application::auth::keys::load_key;
use crate::utils::shared::AppError;

pub fn encrypt_token(key: &[u8;32],data:&[u8])
->  Result<Vec<u8>,AppError> {
    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));

    let mut nonce = [0u8; 12];
    OsRng.fill_bytes(&mut nonce);

    let nonce_obj = Nonce::from_slice(&nonce);

    let mut ciphertext = cipher.encrypt(
        nonce_obj,data)
        .map_err(|_| AppError::Unauthorized("encrypt error".to_string()))?;


  let mut blob = Vec::with_capacity(12 + ciphertext.len());
    blob.extend_from_slice(&nonce);
    blob.append(&mut ciphertext);
    Ok(blob)
}
pub fn decrypt_token(
    key: &[u8; 32],
    blob: &[u8],
) -> Result<Vec<u8>, AppError> {

    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));

    let (nonce, ciphertext) = blob.split_at(12);

    let nonce = Nonce::from_slice(nonce);

    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| AppError::Internal("decrypt failed".into()))
}

pub fn api_token_encrypt(token:&str)->Result<String,AppError>{
    let key = load_key()?;
    let encrypted = encrypt_token(&key,token.as_bytes()).map_err(|e| AppError::Unauthorized(e.to_string()))?;
    Ok(general_purpose::STANDARD.encode(&encrypted))
}


pub  fn api_token_decrypt(token:&str)->Result<String,AppError>{
    let key = load_key()?;
    let decoded = base64::engine::general_purpose::STANDARD.decode(token)
        .map_err(|_| AppError::Internal("decode api_token_decrypt error".to_string()))?;

    let decrypted = decrypt_token(&key,&decoded).map_err(|e| AppError::Unauthorized(e.to_string()))?;

    let jwt = String::from_utf8(decrypted)
        .map_err(|_| AppError::Internal("decode jwt error".to_string()))?;
    Ok(jwt)
}