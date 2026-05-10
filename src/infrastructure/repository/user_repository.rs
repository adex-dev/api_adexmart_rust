use sqlx::{Pool, Postgres};

use crate::domain::user::UserEntity;
use crate::helpers::commons::splits_prefix_module;

pub async fn find_by_username(
    db: &Pool<Postgres>,
    username: &str,
) -> Result<UserEntity, sqlx::Error> {
    let query = format!("{} WHERE username = '{}'",select_user_query(), username);
    let user = sqlx::query_as::<_, UserEntity>(&query)
    .fetch_one(db)
    .await?;
    Ok(user)
}
fn select_user_query() -> &'static str {
    r#"SELECT
        id,
        id_user,
        firstname,
        lastname,
        fullname,
        username,
        roles,
        status,
        created_at,
        modifed_at,
        store_id,
          password
    FROM users "#
}
pub async fn find_by_id(
    db: &Pool<Postgres>,
    id_user: &str,
) -> Result<UserEntity, sqlx::Error> {
    let(column,value) = if splits_prefix_module("E",id_user) {
        ("id_user",id_user)
    }else {
        ("id",id_user)
    };
    let query = format!("{} WHERE {} = $1",select_user_query(), column);
    let user = sqlx::query_as::<_, UserEntity>(&query)
        .bind(value)
        .fetch_one(db)
        .await?;
    Ok(user)
}
