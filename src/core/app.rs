use reqwest::blocking::Client;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::user_agent::generate::generate_ua;
pub const VERSION: &str = "stable-0.1-1";
#[derive(Deserialize)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

pub fn get_latest_github_release() -> Result<String, Box<dyn Error>> {
    let url = "https://api.github.com/repos/ros-e/lumi/releases/latest";

    let client = Client::new();
    let response = client
        .get(url)
        .header("User-Agent", generate_ua())
        .send()?
        .error_for_status()?;

    let release: Release = response.json()?;
    Ok(release.tag_name)
}

pub fn download_latest_github_release() -> Result<String, Box<dyn Error>> {
    let url = "https://api.github.com/repos/ros-e/lumi/releases/latest";
    let client = Client::new();
    let response = client
        .get(url)
        .header("User-Agent", generate_ua())
        .send()?
        .error_for_status()?;
    let release: Release = response.json()?;
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    let asset = release.assets.iter().find(|asset| {
        let name_lower = asset.name.to_lowercase();
        (match os {
            "windows" => name_lower.contains("windows") || name_lower.contains("win") || name_lower.ends_with(".exe"),
            "macos" => name_lower.contains("macos") || name_lower.contains("darwin") || name_lower.contains("mac"),
            "linux" => name_lower.contains("linux"),
            _ => false,
        }) && match arch {
            "x86_64" => name_lower.contains("x86_64") || name_lower.contains("amd64") || name_lower.contains("x64"),
            "aarch64" => name_lower.contains("aarch64") || name_lower.contains("arm64"),
            "x86" => name_lower.contains("x86") || name_lower.contains("i386") || name_lower.contains("i686"),
            _ => true, 
        }
    }).ok_or("No suitable release asset found for your platform")?;
    println!("Downloading: {}", asset.name);
    let download_response = client
        .get(&asset.browser_download_url)
        .header("User-Agent", generate_ua())
        .send()?
        .error_for_status()?;
    let downloads_dir = "downloads";
    std::fs::create_dir_all(downloads_dir)?;
    let file_path = Path::new(downloads_dir).join(&asset.name);
    let mut file = File::create(&file_path)?;
    let content = download_response.bytes()?;
    file.write_all(&content)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = file.metadata()?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&file_path, perms)?;
    }
    Ok(file_path.to_string_lossy().to_string())
}