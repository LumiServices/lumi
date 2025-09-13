use std::{fs::create_dir};

use clap::{Parser, Subcommand};
use colored::*;
use lumi::{
    api,
    core::{self, app::get_latest_github_release},
    discord::webhook::{Embed, webhook_request},
    s3::credentials::{generate_access_key, generate_secret_key},
};

#[derive(Parser, Debug)]
#[command(version = core::app::VERSION, about = "lumi CLI")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Serve {
        #[arg(default_value = "8080")]
        port: u64,
        #[arg(long, action = clap::ArgAction::SetTrue)]
        hide_banner: bool,
        #[arg(long)]
        webhook_url: Option<String>,
    }, 
    Update,
    GenerateCredentials {
        #[arg(long, short = 'a', default_value_t = 20)]
        access_key_length: usize,
        #[arg(long, short = 's', default_value_t = 40)]
        secret_key_length: usize,
    },
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    //create data directory if it doesnt already exist
    create_dir("./data").or_else(|e| {
        if e.kind() == std::io::ErrorKind::AlreadyExists {
            Ok(())
        } else {
            Err(e)
        }
    })?;
    // TODO: auto check for updates (if enabled in config)
    let args = Args::parse();
    match args.command {
        Commands::Serve { port, hide_banner, webhook_url } => {
        if let Some(url) = webhook_url {
            let mut embed = Embed::new();
            embed.title = Some("🚀 Lumi Server Started!".to_string());
    embed.description = Some(format!(
        "```ansi\n\u{001b}[1;34mVersion: {}\nStatus: Online\u{001b}[0m\n```\n🔗 **Repository:** [ros-e/lumi](https://github.com/ros-e/lumi)", 
        core::app::VERSION,
    ));
    embed.color = Some(0x6291FF);
            let embeds = vec![embed];
            let result = webhook_request(
                Some("lumi".to_string()),
                Some("https://github.com/ros-e/lumi/blob/main/src/discord/avatar.png?raw=true".to_string()),
                None,
                embeds,
                url
            ).await;
            
            if let Err(e) = result {
                eprintln!("Failed to send webhook! {}", e);
            }
        }

    api::server::start_server(port, !hide_banner).await;
    Ok(())
}
        Commands::Update => {
            println!("{}", "Checking for updates...".yellow());
            match get_latest_github_release() {
                Ok(latest) => {
                    let current = core::app::VERSION.trim_start_matches('v');
                    let latest = latest.trim_start_matches('v');
                    if latest != current {
                        println!(
                            "{}",
                            format!(
                                "New version available: {} (current: {})",
                                latest, current
                            )
                            .cyan()
                        );
                    } else {
                        println!("{}", "You are running the latest version.".green());
                    }
                }
                Err(e) => {
                    println!(
                        "{}",
                        format!("Failed to check for updates: {}", e).red()
                    );
                }
            }
            Ok(())
        }
        Commands::GenerateCredentials { access_key_length, secret_key_length } => {
            match generate_access_key(Some(access_key_length)) {
                Ok(access_key) => {
                    match generate_secret_key(Some(secret_key_length)) {
                        Ok(secret_key) => {
                            println!("Access Key: {}", access_key.bright_blue());
                            println!("Secret Key: {}", secret_key.bright_blue());
                            println!("\n{}", "Store these credentials securely!".yellow());
                            Ok(())
                        }
                        Err(e) => {
                            eprintln!("{}", format!("Failed to generate secret key: {}", e).red());
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("{}", format!("Failed to generate access key: {}", e).red());
                    std::process::exit(1);
                }
            }
        }
    }
}