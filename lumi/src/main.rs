use clap::{Parser, Subcommand};
use lumi_server::{http::start_http_server};
use lumi_utils::update::fetch_latest_github_release;
use std::io::Write;
#[derive(Parser, Debug)]
#[command(version = env!("CARGO_PKG_VERSION"), about = "lumi CLI")]
struct Args {
     #[command(subcommand)]
     command: Commands,
}
#[derive(Subcommand, Debug)]
enum Commands {
    Serve {
        #[arg(long, default_value = "0.0.0.0")]
        host: String,
        #[arg(default_value = "8080")]
        port: u64,
        #[arg(long, default_value = "*")]
        allowed_origin: String,
        #[arg(long)]
        logs: bool,
    }, 
    Update,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    match args.command {
        Commands::Serve { host, port, allowed_origin, logs } => {
                start_http_server(host, port, allowed_origin, logs).await?;
                Ok(())
            }
        Commands::Update => {
            println!("\x1b[33mChecking for updates...\x1b[0m");
            match tokio::task::spawn_blocking(|| {
                fetch_latest_github_release().map_err(|e| e.to_string())
            }).await {
                Ok(result) => match result {
                    Ok(latest) => {
                        let current = env!("CARGO_PKG_VERSION");
                        let latest = latest.trim();
                        if latest != current {
                            println!("\x1b[36mNew version available: {} (current: {})\x1b[0m", latest, current);
                            println!("Visit https://github.com/ros-e/lumi/releases/latest to download");
                        } else {
                            println!("\x1b[32mYou are running the latest version.\x1b[0m");
                        }
                    }
                    Err(e) => {
                        println!("\x1b[31mFailed to check for updates: {}\x1b[0m", e);
                    }
                },
                Err(e) => {
                    println!("\x1b[31mFailed to spawn blocking task: {}\x1b[0m", e);
                }
            }
            Ok(())
        }
    }
}