use std::collections::HashMap;
use axum::{
    extract::{Path}, 
    http::{HeaderMap, HeaderValue, StatusCode}, 
    response::IntoResponse,
    body::Body,
};
use lumi_utils::errors::ErrorCode;
use tokio::fs::File;
use tokio_util::io::ReaderStream;

pub async fn get_object_handler(
    Path(params): Path<HashMap<String, String>>, 
) -> impl IntoResponse {
    let bucket = match params.get("bucket") {
        Some(b) => b,
        None => return ErrorCode::NoSuchBucket.into_response(),
    };
    let key = match params.get("key") {
        Some(k) => k,
        None => return ErrorCode::InvalidRequest.into_response(),
    };
    let file_path = format!("./data/{}/{}", bucket, key);
    let file = match File::open(&file_path).await {
        Ok(f) => f,
        Err(_) => return ErrorCode::NoSuchKey.into_response(),
    };
    let mut headers = HeaderMap::new();
    let content_type = match key.rsplit('.').next() {
        Some("html") => "text/html",
        Some("json") => "application/json",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        Some("txt") => "text/plain",
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        Some("pdf") => "application/pdf",
        _ => "application/octet-stream",
    };
    headers.insert("content-type", HeaderValue::from_static(content_type));
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);
    (StatusCode::OK, headers, body).into_response()
}