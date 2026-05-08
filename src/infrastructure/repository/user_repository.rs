use sqlx::{Pool, Postgres};

use crate::domain::user::{UserEntity};

pub async fn find_by_username(
    db: &Pool<Postgres>,
    username: &str,
) -> Result<UserEntity, sqlx::Error> {
    let user = sqlx::query_as::<_, UserEntity>(
        r#"
    SELECT
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
    FROM users
    WHERE username = $1
    "#,
    )
    .bind(username)
    .fetch_one(db)
    .await?;
    Ok(user)
}