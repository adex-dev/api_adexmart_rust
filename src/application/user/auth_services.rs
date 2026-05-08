use sqlx::{Pool, Postgres};

use crate::{
    application::auth::jwt_services::{generate_access_token}, domain::{
        LoginRequest,
        store_entity::LoginStore,
        user::{LoginResponse, UserAccessResponse, UserResponse},
    }, helpers::commons, infrastructure::{auth_repository, store_repository, user_repository}, utils::password::check_password
};

pub async fn login_service(db: &Pool<Postgres>, payload: LoginRequest) -> Result<LoginResponse, String> {
    let user = user_repository::find_by_username(db, &payload.username)
        .await
        .map_err(|_| "User tidak ditemukan".to_string())?;

    let valid_password = check_password(&user.password, &payload.password)
        .map_err(|_| "Password gagal diverifikasi".to_string())?;
    if !valid_password {
        return Err("Password salah".to_string());
    }
    if &user.status != "active" {
        let message = match user.status.as_str() {
            "inactive" => "user sudah tidak aktif",
            "suspend" => "user telah di suspend",
            "block" => "akses user sudah dihapus",
            _ => "akses ditolak Modul",
        };
        return Err(message.to_string());
    }
    let required_access = ["cashier", "supervisor"];
    let is_access_restricted = required_access.contains(&user.roles.as_str());
    let user_access = auth_repository::find_user_access(db, &user.id_user)
        .await
        .unwrap_or(vec![]);

    if user_access.is_empty() && is_access_restricted {
        return Err("Akses ditolak".to_string());
    }
    let is_store_restricted = required_access.contains(&user.roles.as_str());
    if !is_store_restricted && commons::is_empty(&user.store_id) {
        return  Err("Akses tidak di izinkan anda wajib memasukan store login".to_string());
    }
    let store_access =store_repository::find_store_by_id(db, &user.store_id)
    .await
     .unwrap_or(vec![]);

    let access_token = generate_access_token(&user.id_user, &user.roles)?;

    Ok(LoginResponse {
        user: UserResponse {
            id: user.id,
            id_user: user.id_user,
            firstname: user.firstname,
            lastname: user.lastname,
            fullname: user.fullname,
            username: user.username,
            roles: user.roles,
            status: user.status,
            store_id: user.store_id,
            created_at: user.created_at,
            modifed_at: user.modifed_at,
        },
        token: access_token,
        modul_access: user_access
            .into_iter()
            .map(|m| UserAccessResponse {
                id: m.id,
                name: m.name,
            })
            .collect(),
        store: store_access.into_iter().map(|s| LoginStore{
            store_id:s.store_id,
            name:s.name,
            address:s.address,
            cities:s.cities,
            province:s.province,
            phone1:s.phone1,
            phone2:s.phone2,
            parent_area:s.parent_area,
        }).collect(),
    })
}
