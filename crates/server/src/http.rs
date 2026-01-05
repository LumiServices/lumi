use axum::{
    Router,
};
use tower_http::{
    cors::{CorsLayer, Any},
    trace::TraceLayer,
};
use std::{error::Error, net::SocketAddr};

pub async fn start_http_server(
    host: String,
    port: u64,
    allowed_origins: String,
    logs: bool,
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
    let mut routes = Router::new();
    routes = routes.layer(cors_layer); 
    if logs {
        routes = routes.layer(TraceLayer::new_for_http());
    }
    
    axum::serve(listener, routes).await?;
    Ok(())
}