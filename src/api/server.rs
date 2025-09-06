use axum::{
    Router, middleware,
};

use crate::s3::middleware::s3_auth_middleware;


#[tokio::main]
pub async fn start_server(port: u64, show_start_banner: bool) {
    if show_start_banner {
        println!("REST API started on http://0.0.0.0:{}", port);
    }
    
    let app = Router::new()
        .layer(middleware::from_fn(s3_auth_middleware));
    
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    
    axum::serve(listener, app).await.unwrap();
}