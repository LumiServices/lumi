use reqwest::blocking::Client;
use serde::Deserialize;
use std::error::Error;
pub const VERSION: &str = "beta-0.0.1-patch_1.1";
#[derive(Deserialize)]
struct Release {
    tag_name: String,
}

pub fn get_latest_github_release() -> Result<String, Box<dyn Error>> {
    let url = "https://api.github.com/repos/ros-e/lumi/releases/latest";

    let client = Client::new();
    let response = client
        .get(url)
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/86.0.4240.111 Safari/537.36 Edg/86.0.622.56")
        .send()?
        .error_for_status()?;

    let release: Release = response.json()?;
    Ok(release.tag_name)
}
