use axum::{
    Router, middleware, routing::{get, put},
};
use crate::{ api::objects::{get_object_handler, put_object_handler}, s3::middleware::s3_auth_middleware};

#[allow(unused_variables)] // <- remove after adding banner :3
#[tokio::main]
pub async fn start_server(port: u64, show_start_banner: bool) {
        println!("REST API started on http://0.0.0.0:{}", port);
    
    let app = Router::new()
         .route("/{bucket}/{*key}", put(put_object_handler))
        .layer(middleware::from_fn(s3_auth_middleware))
         .route("/{bucket}/{*key}", get(get_object_handler));
    
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    
    axum::serve(listener, app).await.unwrap();
}