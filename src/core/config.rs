use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub server: ServerConfig,
    pub discord: DiscordConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    pub port: u16,
    pub data_dir: String,
    pub db_dir: String, 
    pub allowed_origin: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DiscordConfig {
    pub webhook_url: String,
    pub logging: LoggingConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoggingConfig {
    pub server: ServerLogging,
    pub uploaded: UploadedLogging,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerLogging {
    pub startup: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UploadedLogging {
    pub images: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            server: ServerConfig {
                port: 8080,
                data_dir: "./data".to_string(),
                db_dir: "./db".to_string(),
                allowed_origin: "*".to_string(),
            },
            discord: DiscordConfig {
                webhook_url: String::new(),
                logging: LoggingConfig {
                    server: ServerLogging {
                        startup: true,
                    },
                    uploaded: UploadedLogging {
                        images: true,
                    },
                },
            },
        }
    }
}
pub fn read_config(location: String) -> Result<Config, Box<dyn std::error::Error>> {
    match fs::read_to_string(&location) {
        Ok(contents) => {
            let config: Config = serde_yaml::from_str(&contents)?;
            println!("Loaded config from: {}", location);
            Ok(config)
        }
        Err(_) => {
            println!("Config file not found at: {}", location);
            println!("Using default configuration");
            let default_config = Config::default();
            if let Ok(yaml) = serde_yaml::to_string(&default_config) {
                if fs::write(&location, yaml).is_ok() {
                    println!("Created default config at: {}", location);
                }
            }
            
            Ok(default_config)
        }
    }
}
pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    read_config("config.yaml".to_string())
}