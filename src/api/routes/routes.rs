use axum::{
    middleware,
    routing::{get, post,put},
    Router,
    http::{
        Method,
        header::{
            AUTHORIZATION,
            ACCEPT,
            CONTENT_TYPE,
        }
    }
};
use tower_http::cors::{ CorsLayer};
use crate::{api::handlers::login, state::AppState, api::handlers::refresh, api::middleware::jwt::jwt_middleware};
use crate::api::handlers::auth_handler::refresh_full;

pub fn create_routes() ->Router<AppState>{
    let cors =CorsLayer::new()
        .allow_origin([
            "http://localhost:5173".parse().unwrap(),
            "http://127.0.0.1:5173".parse().unwrap(),
        ])
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::OPTIONS,
            Method::DELETE,
        ]).allow_headers(
        [
            AUTHORIZATION,
            CONTENT_TYPE,
            ACCEPT
        ])
        .allow_credentials(true);
     let logi  = Router::new()
    .route("/login",post(login));
    let auth_routes = Router::new()
        .route("/refresh", post(refresh))
        .route("/refressh", post(refresh_full));

    let private = Router::new()
        .route("/profile",put(profileput()))
        .route("/profile",get(profileput()))
        .route_layer(middleware::from_fn(jwt_middleware));
    Router::new().nest("/api", logi).nest("/api/v1",private).nest("/api/v1/auth",auth_routes).layer(cors)
}

pub fn profileput(){
    println!("profile");
}
