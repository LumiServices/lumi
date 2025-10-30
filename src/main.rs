use std::{fs::create_dir_all, io::Write};
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use colored::*;
use lumi::core::app::download_latest_github_release;
use lumi::{
    api,
    core::{self, app::get_latest_github_release},
    discord::webhook::{Embed, webhook_request},
    s3::credentials::{generate_access_key, generate_secret_key},
    db::sqlite::{Database, DB},
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
        #[arg(long, default_value = "*")]
        allowed_origin: String,
    }, 
    Update,
    GenerateCredentials {
        #[arg(long, short = 'a', default_value_t = 20)]
        access_key_length: usize,
        #[arg(long, short = 's', default_value_t = 40)]
        secret_key_length: usize,
    },
    SetBucketPolicy {
        bucket: String,
        #[arg(value_parser = ["private", "public-read", "public"])]
        policy: String,
    },
    GetBucketPolicy {
        bucket: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data_dir = "./data";
    let db_dir = "./db";
    
    create_dir_all(data_dir)?;
    create_dir_all(db_dir)?;
    
    let db_path = PathBuf::from("./db/lumi.db");
    let db = Database::new(&db_path)?;
    db.create_table("metadata", "object_key", "content_type")?;
    db.create_table("bucket_policies", "bucket_name", "policy")?;
    DB.set(db).expect("Failed to initialize database");
    
    let args = Args::parse();
    match args.command {
        Commands::Serve { port, hide_banner, webhook_url, allowed_origin } => {
            if let Some(url) = &webhook_url {
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
                    url.clone()
                ).await;
                
                if let Err(e) = result {
                    eprintln!("Failed to send webhook! {}", e);
                }
            }

            api::server::start_server(port, !hide_banner, allowed_origin).await;
            Ok(())
        }
        Commands::Update => {
            println!("{}", "Checking for updates...".yellow());
            match tokio::task::spawn_blocking(|| {
                get_latest_github_release().map_err(|e| e.to_string())
            }).await {
                Ok(result) => match result {
                    Ok(latest) => {
                        let current = core::app::VERSION.trim_start_matches('v');
                        let latest = latest.trim_start_matches('v');
                        if latest != current {
                            println!("{}", format!("New version available: {} (current: {})", latest, current).cyan());
                            tokio::task::spawn_blocking(|| {
                                loop {
                                    print!("Would you like to download the latest version? (Y/N): "); 
                                    std::io::stdout().flush().expect("flush failed");
                                    let mut input = String::new();
                                    std::io::stdin().read_line(&mut input).expect("failed to read line");
                                    let input = input.trim().to_lowercase();
                                    match input.as_str() {
                                        "y" => {
                                            match download_latest_github_release() {
                                                Ok(path) => {
                                                    println!("Downloaded successfully to {}", path);
                                                }
                                                Err(e) => {
                                                    eprintln!("Failed to download update: {}", e);
                                                }
                                            }
                                            break;
                                        }
                                        "n" => {
                                            break;
                                        }
                                        _ => {
                                            println!("Please enter 'Y' or 'N'.");
                                        }
                                    }
                                }
                            }).await.unwrap();
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
                },
                Err(e) => {
                    println!(
                        "{}",
                        format!("Failed to spawn blocking task: {}", e).red()
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
        Commands::SetBucketPolicy { bucket, policy } => {
        println!("{}", "Coming soon ".yellow());
        Ok(())
        }  

    Commands::GetBucketPolicy { bucket } => {
        println!("{}", "Coming soon ".yellow());
        Ok(())
    }
    }
}