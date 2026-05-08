use sqlx::{Pool,Postgres};

use crate::domain::user::UserAccessEntity;


pub async fn find_user_access(
    db:&Pool<Postgres>,
    id_user:&str,
)->Result<Vec<UserAccessEntity>,sqlx::Error>{
    let access = sqlx::query_as::<_,UserAccessEntity>(
        r#"
        SELECT p.id,m.name
        FROM user_permissions as p
        left join modules as m on m.id = p.module_id 
        WHERE 1=1 externalid_user=$1
        "#
    ).bind(id_user)
    .fetch_all(db)
    .await?;
    Ok(access)
}
