pub mod repository{
    pub mod user_repository;
    pub mod store_repository;
    pub mod auth_repository;
}

pub use repository::user_repository;
pub use repository::store_repository;
pub use repository::auth_repository;