use axum::{
    Router, serve
};

pub async fn start_server(port: u64, allowed_origin: String) {
    println!("REST API started on http://0.0.0.0:{}", port);
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    let routes = Router::new();
    serve(listener, routes).await.unwrap();
}