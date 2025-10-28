use axum::{
    Json, Router, http::HeaderValue, middleware::{self}, routing::{delete, get, put}
};
use serde_json::json;
use crate::{ 
    api::{
        buckets::{create_bucket_handler, delete_bucket_handler, list_buckets_handler}, objects::{
            delete_object_handler, 
            get_object_handler, 
            list_objects_v2_handler, 
            put_object_handler
        }
    }, 
    s3::middleware_v2::s3_auth_middleware
};
use tower_http::cors::{CorsLayer, Any};

#[allow(unused_variables)] // <- remove after adding banner :3
pub async fn start_server(port: u64, show_start_banner: bool, allowed_origin: String) {
    println!("REST API started on http://0.0.0.0:{}", port);
    
    let cors = CorsLayer::new()
        .allow_origin(
            allowed_origin
                .parse::<HeaderValue>()
                .expect("Invalid origin header value")
        )
        .allow_methods(Any)
        .allow_headers(Any); 
    
    let app = Router::new()
        .route("/", get(list_buckets_handler))
        .route("/{bucket}/{*key}", put(put_object_handler))
        .route("/{bucket}/{*key}", delete(delete_object_handler))
        .route("/{bucket}/", put(create_bucket_handler))
        .route("/{bucket}/", delete(delete_bucket_handler))
        .route("/{bucket}/", get(list_objects_v2_handler))
        .layer(middleware::from_fn(s3_auth_middleware))
        .route("/health", get(health))
        .route("/{bucket}/{*key}", get(get_object_handler))
        .layer(cors);
    
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> Json<serde_json::Value> {
    Json(json!({ "success": true }))
}