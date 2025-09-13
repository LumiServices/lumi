use std::error::Error;
use reqwest::Client;
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
pub struct EmbedFooter {
    pub text: String,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

#[derive(Serialize)]
pub struct EmbedImage {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<u32>,
    pub width: Option<u32>,
}

#[derive(Serialize)]
pub struct EmbedThumbnail {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<u32>,
    pub width: Option<u32>,
}

#[derive(Serialize)]
pub struct EmbedVideo {
    pub url: Option<String>,
    pub proxy_url: Option<String>,
    pub height: Option<u32>,
    pub width: Option<u32>,
}

#[derive(Serialize)]
pub struct EmbedProvider {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Serialize)]
pub struct EmbedAuthor {
    pub name: String,
    pub url: Option<String>,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

#[derive(Serialize)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline: Option<bool>,
}

#[derive(Serialize)]
pub struct Embed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub embed_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<EmbedFooter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<EmbedImage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<EmbedThumbnail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<EmbedVideo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<EmbedProvider>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<EmbedAuthor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<EmbedField>>,
}

impl Embed {
    pub fn new() -> Self {
        Self {
            title: None,
            embed_type: Some("rich".to_string()),
            description: None,
            url: None,
            color: None,
            footer: None,
            image: None,
            thumbnail: None,
            video: None,
            provider: None,
            author: None,
            fields: None,
        }
    }
}

pub async fn webhook_request(
    username: Option<String>,
    avatar_url: Option<String>,  
    content: Option<String>,
    embeds: Vec<Embed>,
    url: String,
    
) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let body = json!({
        "username": username,
        "avatar_url": avatar_url,
        "content": content,
        "embeds": embeds,
    });
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    Ok(response.text().await?)
}

#[derive(Serialize)]
pub struct Message {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<Embed>>,
}