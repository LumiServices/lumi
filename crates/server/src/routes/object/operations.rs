use axum::{
    body::{Body, Bytes},
    http::{HeaderMap, HeaderValue, Method, StatusCode},
    response::{IntoResponse, Response},
};
use lumi_utils::errors::ErrorCode;
use tokio::fs::File;
use tokio_util::io::ReaderStream;

pub async fn handle(
    method: Method,
    bucket: String,
    key: String,
    headers: HeaderMap,
    _body: Bytes,
) -> Response {
    match method {
        Method::GET => get_object(bucket, key, headers).await.into_response(),
        _ => ErrorCode::MethodNotAllowed.into_response(),
    }
}

pub async fn get_object(
    bucket: String,
    key: String,
    _req_headers: HeaderMap,
) -> impl IntoResponse {
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
