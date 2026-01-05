use std::error::Error;
use reqwest::blocking::Client;

pub fn fetch_latest_github_release() -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let response = client
        .get("https://api.github.com/repos/ros-e/lumi/releases/latest")
        .header("User-Agent", "lumi-client")
        .send()?
        .error_for_status()?;
    
    let release: serde_json::Value = response.json()?;
    let tag_name = release["tag_name"]
        .as_str()
        .ok_or("tag_name not found")?
        .to_string();
    
    Ok(tag_name)
}