use clap::{Parser, Subcommand};
use lumi::server::start_server;

#[derive(Parser, Debug)]
#[command(name = "lumi")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
/*
COMMANDS:
    serve                --port <port>
    set-bucket-policy    --bucket <bucket> --policy <policy>
    get-bucket-policy    --bucket <bucket>
*/
#[derive(Subcommand, Debug)]
enum Commands {
    Serve {
        #[arg(long, default_value = "8080")]
        port: u16,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Serve { port } => {
            if let Err(err) = start_server(port).await {
                eprintln!("error starting lumi-server: {}", err);
            }
        }
    }
}