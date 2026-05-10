use sqlx::{PgPool,postgres::PgPoolOptions};
pub fn database_url() -> String{
    format!(
        "postgres://{}:{}@{}:{}/{}?ssl_mode={}",
        std::env::var("DB_USER").unwrap(),
        std::env::var("DB_PASSWORD").unwrap(),
        std::env::var("DB_HOST").unwrap(),
        std::env::var("DB_PORT").unwrap(),
        std::env::var("DB_NAME").unwrap(),
        std::env::var("DB_SSLMODE").unwrap(),
    )
}
pub async fn connect_db() -> PgPool{

    // db_user,db_password,db_host,db_port,db_name,db_ssl
    // println!("Database URL:{}",database_url);
    let db_url = database_url();
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect database")
}

