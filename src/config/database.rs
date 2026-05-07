use sqlx::{PgPool,postgres::PgPoolOptions};

pub async fn connect_db() -> PgPool{
    let db_host = std::env::var("DB_HOST").expect("DB_HOST not found");
    let db_port = std::env::var("DB_PORT").expect("DB_PORT not found");
    let db_user = std::env::var("DB_USER").expect("DB_USER not found");
    let db_name = std::env::var("DB_NAME").expect("DB_NAME not found");
    let db_password = std::env::var("DB_PASSWORD").expect("DB_PASSWORD not found");
    let db_ssl = std::env::var("DB_SSLMODE").unwrap_or("disable".to_string());

    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}?ssl_mode={}",
        db_user,db_password,db_host,db_port,db_name,db_ssl
    );
    println!("Database URL:{}",database_url);
    PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await
    .expect("Failed to connect database")
}