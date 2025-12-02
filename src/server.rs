use axum::{Router};

pub async fn start_server(port: u16) -> Result<(), std::io::Error> {
    println!("lumi started on http://0.0.0.0:{}", port);
    let server = Router::new();
    axum::serve(
tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?,
        server.into_make_service(),
    )
    .await
}

