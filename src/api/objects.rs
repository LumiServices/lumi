use std::collections::HashMap;
use axum::{
    extract::Path, 
    response::IntoResponse,
    http::StatusCode,
};
use tokio::fs;
use crate::s3::errors::ErrorCode;

pub async fn put_object_handler() -> impl IntoResponse {
    StatusCode::OK
}

pub async fn get_object_handler(
    Path(params): Path<HashMap<String, String>>
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
    match fs::read(&file_path).await {
        Ok(contents) => {
            (StatusCode::OK, contents).into_response()
        },
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::NotFound => ErrorCode::NoSuchKey.into_response(),
                _ => ErrorCode::InternalError.into_response(),
            }
        }
    }
}