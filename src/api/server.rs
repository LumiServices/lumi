use axum::{
    Router, middleware, routing::{get, put},
};
use crate::{ api::{buckets::create_bucket_command_handler, objects::{get_object_handler, list_objects_v2_handler, put_object_handler}}, s3::middleware::s3_auth_middleware};
use tower_http::cors::{CorsLayer, Any};
#[allow(unused_variables)] // <- remove after adding banner :3
#[tokio::main]
pub async fn start_server(port: u64, show_start_banner: bool) {
        println!("REST API started on http://0.0.0.0:{}", port);
        let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any); 
    let app = Router::new()
         .route("/{bucket}/{*key}", put(put_object_handler))
         .route("/{bucket}", put(create_bucket_command_handler))
         .route("/{bucket}", get(list_objects_v2_handler))
        .layer(middleware::from_fn(s3_auth_middleware))
         .route("/{bucket}/{*key}", get(get_object_handler))
         .layer(cors);
    
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    
    axum::serve(listener, app).await.unwrap();
}