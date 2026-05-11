use sqlx::{PgPool, Pool, Postgres};
use uuid::Uuid;
use crate::domain::user::{TokenRefreshEntity, UserAccessEntity};
use crate::utils::shared::AppError;

pub async fn find_user_access(
    db: &Pool<Postgres>,
    id_user: &str,
) -> Result<Vec<UserAccessEntity>, sqlx::Error> {
    let access = sqlx::query_as::<_, UserAccessEntity>(
        r#"
        SELECT p.id,m.name
        FROM user_permissions as p
        left join modules as m on m.id = p.module_id 
        WHERE 1=1 externalid_user=$1
        "#,
    )
    .bind(id_user)
    .fetch_all(db)
    .await?;
    Ok(access)
}

pub async fn find_token(
    db: &PgPool,
    jti: &str,
) -> Result<TokenRefreshEntity, AppError> {
    let jti = parse_jti(jti)?;
    let row = sqlx::query_as::<_, TokenRefreshEntity>(
        r#"
        SELECT token_hash, expires_at, revoked
        FROM refresh_tokens
        WHERE jti= $1
        "#
).bind(jti).fetch_one(db).await
        .map_err(|e| AppError::Validation(e.to_string()))?;
    Ok(row)
}

pub async fn delete_token(db: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        DELETE FROM refresh_tokens WHERE expires_at < NOW()
"#,
    )
    .execute(db)
    .await?;
    Ok(())
}
pub async fn save_refresh_token(
    db: &Pool<Postgres>,
    jti: &Uuid,
    user_id: &str,
    token: &str,
    expires_in: &chrono::NaiveDateTime,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
INSERT INTO refresh_tokens(jti,user_id,token_hash,expires_at,revoked)
VALUES ($1,$2,$3,$4,FALSE)
"#,
    )
    .bind(jti)
    .bind(user_id)
    .bind(token)
    .bind(expires_in)
    .execute(db)
    .await?;
    Ok(())
    }

pub async fn logout_token(
    db: &Pool<Postgres>,
    jti: &str,
) -> Result<(), AppError> {
    let jti = parse_jti(jti)?;
    sqlx::query(
        r#"
        UPDATE refresh_tokens SET revoked=TRUE WHERE jti = $1"#
    ).bind(jti).execute(db)
        .await
        .map_err(|e| AppError::Validation(e.to_string()))?;
    Ok(())
}


pub fn parse_jti(value: &str) -> Result<Uuid, AppError> {
    Uuid::parse_str(value)
        .map_err(|_| AppError::BadRequest("invalid uuid".to_string()))
}