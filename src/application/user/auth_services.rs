use sqlx::{Pool, Postgres};

use crate::domain::user::{LoginResult, TokenEntity, UserAccessEntity, UserEntity};
use crate::infrastructure::auth_repository::{delete_token, logout_token, save_refresh_token};
use crate::utils::shared::AppError;
use crate::{
    application::auth::{
        jwt_services::generate_access_token,
        jwt_services::generate_refresh_token,
        crypto::api_token_encrypt
    },
    domain::{
        LoginRequest,
        user::{LoginResponse},
    },
    helpers::commons,
    infrastructure::{auth_repository, store_repository, user_repository},
    utils::password::check_password,
};
use crate::domain::store_entity::StoreEntity;
use crate::utils::jwt::verify_refresh_token;

pub async fn login_service(
    db: &Pool<Postgres>,
    payload: LoginRequest,
) -> Result<LoginResult, AppError> {
    let user = user_repository::find_by_username(db, &payload.username)
        .await
        .map_err(|_| AppError::NotFound("User tidak ditemukan".to_string()))?;
    let hashed_password = user.password.clone();
    let plain_password = payload.password.clone();
    let valid_password = tokio::task::spawn_blocking(move||{
        check_password(&hashed_password, &plain_password)
    }).await
            .map_err(|_| AppError::Unauthorized("Task Gagal".to_string()))?
            .map_err(|_| AppError::Unauthorized("Password gagal diverifikasi".to_string()))?;
    
    if !valid_password {
        return Err(AppError::Unauthorized("Password salah".to_string()));
    }
    if &user.status != "active" {
        let message = match user.status.as_str() {
            "inactive" => "user sudah tidak aktif",
            "suspend" => "user telah di suspend",
            "block" => "akses user sudah dihapus",
            _ => "akses ditolak Modul",
        };
        return Err(AppError::Unauthorized(message.to_string()));
    }
    let required_access = commons::required_access_roles();
    let is_access_restricted = required_access.contains(&user.roles);
    let user_access = auth_repository::find_user_access(db, &user.id_user)
        .await
        .unwrap_or(vec![]);

    if user_access.is_empty() && is_access_restricted {
        return Err(AppError::Unauthorized("Akses ditolak".to_string()));
    }
    let required_access_store = commons::required_access_store_roles();
    let is_store_restricted = required_access_store.contains(&user.roles);

    if !is_store_restricted && commons::is_empty(&user.store_id) {
        return Err(AppError::Unauthorized(
            "Akses tidak di izinkan anda wajib memasukan store login".to_string(),
        ));
    }
    let store_access = store_repository::find_store_by_id(db, &user.store_id)
        .await
        .unwrap_or(vec![]);

    let access_token = generate_access_token(&user.id_user, &user.roles)
        .map_err(|_| AppError::UnauthorizedAuth("Generate token access gagal".to_string()))?;
    let refresh_token = generate_refresh_token(&user.id_user, &user.roles)
        .map_err(|_| AppError::UnauthorizedAuth("Generate token refresh gagal".to_string()))?;
    let _ = save_refresh_token(
        db,
        &refresh_token.jti,
        &refresh_token.user_id.to_string(),
        &refresh_token.token.to_string(),
        &refresh_token.expires_in,
    )
    .await;
    let _ = delete_token(db).await;
    //
    let token_encryption = api_token_encrypt(&access_token)?;

    Ok(LoginResult {
        refresh_token: TokenEntity {
            token: refresh_token.token.to_string(),
        },
        response_data: build_login_response(
            user,
            token_encryption,
            user_access,
            store_access,
        ),
    })
}

pub async fn check_token_refresh(
    db: &Pool<Postgres>,
    jti: &str,
    token: &str,
) -> Result<bool, AppError> {
    let time_now = chrono::Utc::now().naive_utc();
    let find_token = auth_repository::find_token(db, &jti).await.map_err(|e| {
        println!("find_token{}", e);
        AppError::UnauthorizedAuth("checkToken Invalid".to_string())
    })?;
    if find_token.revoked {
        return Err(AppError::UnauthorizedAuth("Token revoked".to_string()));
    }
    if time_now > find_token.expires_at {
        let _ = logout_token(db,jti);
        return Err(AppError::UnauthorizedAuth("Token Expired".to_string()));
    }
    if token != find_token.token_hash {
        return Err(AppError::UnauthorizedAuth("Token mismatch".to_string()));
    }
    Ok(true)
}


pub async fn check_token_refresh_full(
    db: &Pool<Postgres>,
    token: &str,
)-> Result<LoginResponse, AppError> {
    let claims = verify_refresh_token(&token)?;
    let row_token_db = check_token_refresh(db,&claims.jti,token).await
    .map_err(|e| {
        AppError::UnauthorizedAuth(e.to_string())})?;
    if !row_token_db {
        return Err(AppError::UnauthorizedAuth("Token Refresh Not Found".to_string()));
    }
    let access_token = generate_access_token(&claims.sub.to_string(),&claims.roles.to_string())
        .map_err(|_| AppError::UnauthorizedAuth("Generate token access gagal".to_string()))?;
    let token_encryption = api_token_encrypt(&access_token)?;
    let user = user_repository::find_by_id(db, &claims.sub)
        .await
        .map_err(|_| AppError::NotFound("User tidak ditemukan".to_string()))?;
    let required_access = commons::required_access_roles();
    let is_access_restricted = required_access.contains(&user.roles);
    let user_access = auth_repository::find_user_access(db, &user.id_user)
        .await
        .unwrap_or(vec![]);

    if user_access.is_empty() && is_access_restricted {
        return Err(AppError::Unauthorized("Akses ditolak".to_string()));
    }
    let required_access_store = commons::required_access_store_roles();
    let is_store_restricted = required_access_store.contains(&user.roles);

    if !is_store_restricted && commons::is_empty(&user.store_id) {
        return Err(AppError::Unauthorized(
            "Akses tidak di izinkan anda wajib memasukan store login".to_string(),
        ));
    }
    let store_access = store_repository::find_store_by_id(db, &user.store_id)
        .await
        .unwrap_or(vec![]);
    let response = build_login_response(
        user,
        token_encryption,
        user_access,
        store_access,
    );
    Ok(response)
}

pub fn build_login_response(
    user: UserEntity,
    token: String,
    user_access: Vec<UserAccessEntity>,
    store_access: Vec<StoreEntity>,
) -> LoginResponse {

    LoginResponse {
        user: user.into(),
        access_token: token,
        modul_access: user_access.into_iter().map(Into::into).collect(),
        store: store_access.into_iter().map(Into::into).collect(),
    }
}