use axum::{
    Router, routing::get,
};
use tower_http::{
    cors::{CorsLayer, Any},
};
use std::{error::Error, net::SocketAddr};

use crate::routes::{bucket::list_buckets_handler, object::get_object_handler};

pub async fn start_http_server(
    host: String,
    port: u64,
    allowed_origins: String,
) -> Result<(), Box<dyn Error>> {  
    let cors_layer = if allowed_origins == "*" {
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any)
    } else {
        let origins: Vec<_> = allowed_origins
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        CorsLayer::new()
            .allow_origin(origins)
            .allow_methods(Any)
            .allow_headers(Any)
    };
    
    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;
    println!("REST API started on http://{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    let mut routes = Router::new()
     .route("/health", get(health))
     .route("/", get(list_buckets_handler))
     .route("/{bucket}/{*key}", get(get_object_handler));
    routes = routes.layer(cors_layer); 
    axum::serve(listener, routes).await?;
    Ok(())
}

async fn health() -> &'static str {
    "ok"
}