use axum::{
    body::{Body, Bytes},
    http::{HeaderMap, HeaderValue, Method, StatusCode},
    response::{IntoResponse, Response},
};
use lumi_utils::errors::ErrorCode;
use std::path::{Component, Path, PathBuf};
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;
use tokio_util::io::ReaderStream;
pub async fn handle(
    method: Method,
    bucket: String,
    key: String,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    match method {
        Method::GET => get_object(bucket, key, headers).await.into_response(),
        Method::PUT => put_object(bucket, key, headers, body).await.into_response(),
        _ => ErrorCode::MethodNotAllowed.into_response(),
    }
}

pub async fn put_object(
    bucket: String,
    key: String,
    _req_headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    // check if theres an object key
    if key.is_empty() {
        return ErrorCode::NoSuchKey.into_response();
    }
    let mut safe_key = PathBuf::new();
    for component in Path::new(&key).components() {
        match component {
            Component::Normal(part) => safe_key.push(part),
            _ => return ErrorCode::NoSuchKey.into_response(),
        }
    }
    // add some shit to check the content type n prolly some tag shit
    if fs::metadata(&format!("./data/{}", bucket)).await.is_err() {
        return ErrorCode::NoSuchBucket.into_response();
    }
    if let Some(parent) = Path::new(&format!("./data/{}/{}", bucket, safe_key.display())).parent() {
        if let Err(_) = fs::create_dir_all(parent).await {
            return ErrorCode::InternalError.into_response();
        }
    }
    //oki uploaddd :3
    let mut file = match File::create(format!("./data/{}/{}", bucket, safe_key.display())).await {
        Ok(f) => f,
        Err(_) => return ErrorCode::InternalError.into_response(),
    };
    if let Err(_) = file.write_all(&body).await {
        return ErrorCode::InternalError.into_response();
    }
    (StatusCode::OK, {
        let mut headers = HeaderMap::new();
        headers.insert(
            "etag",
            HeaderValue::from_str(&format!("{:x}", md5::compute(&body))).unwrap(),
        );
        headers
    })
        .into_response()
}

pub async fn get_object(bucket: String, key: String, _req_headers: HeaderMap) -> impl IntoResponse {
    let file = match File::open(format!("./data/{}/{}", bucket, key)).await {
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
