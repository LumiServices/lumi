use axum::{middleware::from_fn, Router, routing::any};
use tower_http::cors::{CorsLayer, Any as AnyOrigin};
use std::{env, error::Error, net::SocketAddr};

use crate::middleware;
use crate::routes::{bucket, object};
use lumi_credentials::{DEFAULT_ACCESS_KEY, DEFAULT_SECRET_KEY};

pub async fn start_http_server(
    host: String,
    port: u64,
    allowed_origins: String,
) -> Result<(), Box<dyn Error>> {  
    let cors_layer = if allowed_origins == "*" {
        CorsLayer::new()
            .allow_origin(AnyOrigin)
            .allow_methods(AnyOrigin)
            .allow_headers(AnyOrigin)
    } else {
        let origins: Vec<_> = allowed_origins
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        CorsLayer::new()
            .allow_origin(origins)
            .allow_methods(AnyOrigin)
            .allow_headers(AnyOrigin)
    };
    
    let access_key = env::var("lumi_access_key").unwrap_or_else(|_| DEFAULT_ACCESS_KEY.to_string());
    let secret_key = env::var("lumi_secret_key").unwrap_or_else(|_| DEFAULT_SECRET_KEY.to_string());

    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;
    println!("S3 API started on http://{}:{}", host, port);

    let listener = tokio::net::TcpListener::bind(addr).await?;

    let app = Router::new()
        .route("/", any(bucket::handler))
        .route("/{bucket}/", any(bucket::handler))
        .route("/{bucket}", any(bucket::handler))
        .route("/{bucket}/{*key}", any(object::handler))
        .layer(from_fn(move |req, next| {
            let ak = access_key.clone();
            let sk = secret_key.clone();
            async move { middleware::auth(req, next, &ak, &sk).await }
        }))
        .layer(cors_layer);
    
    axum::serve(listener, app).await?;
    Ok(())
}