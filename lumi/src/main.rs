use::lumi_server::start_server;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    start_server(3000,"*".to_string()).await;
    Ok(())
}