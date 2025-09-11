use serde::Serialize;

#[derive(Serialize)]
pub struct EmbedField {
    name: String,
    value: String,
    inline: Option<bool>,
}

#[derive(Serialize)]
pub struct Embed {
    title: Option<String>,
    description: Option<String>,
    url: Option<String>,
    color: Option<u32>,
    fields: Option<Vec<EmbedField>>,
    footer: Option<EmbedFooter>,
    image: Option<EmbedImage>,
    thumbnail: Option<EmbedThumbnail>,
    author: Option<EmbedAuthor>,
    timestamp: Option<String>,
}

#[derive(Serialize)]
pub struct EmbedFooter {
    text: String,
    icon_url: Option<String>,
}

#[derive(Serialize)]
pub struct EmbedImage {
    url: String,
}

#[derive(Serialize)]
pub struct EmbedThumbnail {
    url: String,
}

#[derive(Serialize)]
pub struct EmbedAuthor {
    name: String,
    url: Option<String>,
    icon_url: Option<String>,
}

pub fn webhook_request(
    username: String,
    content: String,
    embed: Vec<Embed>,
    url: String
) {
    
}