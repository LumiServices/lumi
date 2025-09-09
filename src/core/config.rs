use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub data_dir: String,
    pub db_dir: String, 
    pub allowed_origin: String
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
        }
    }
}

pub fn read_config(location: String) -> std::io::Result<()> {
    println!("{}", location);
    Ok(())
}