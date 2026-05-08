use sqlx::{Pool,Postgres};

use crate::domain::store_entity::StoreEntity;

pub async fn find_store_by_id(
    db:&Pool<Postgres>,
    store_id:&str,
)->Result<Vec<StoreEntity>,sqlx::Error>{
    let store = sqlx::query_as::<_,StoreEntity>(
        r#"
        SELECT 
		s.storeid,
		s.name,
		s.address,
		s.cities,
		s.province,
		s.phone1,
		s.phone2,
		s.status,
		sa.name as parentname,
		s.created_at,
		s.modif_at
	FROM stores s
	LEFT JOIN store_area sa ON s.parent_area = sa.id
	WHERE 1=1 AND s.storeid=$1 LIMIT 1
        "#
    ).bind(store_id)
    .fetch_all(db)
    .await?;
Ok(store)
}