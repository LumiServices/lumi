use std::{collections::HashMap, fs};
use std::path::Path as StdPath;
use axum::{extract::Path, response::IntoResponse};
use reqwest::StatusCode;
use crate::s3::errors::ErrorCode;

pub async fn create_bucket_command_handler(
    Path(params): Path<HashMap<String, String>>,
) -> impl IntoResponse {
    let bucket = match params.get("bucket") {
        Some(b) => b,
        None => return ErrorCode::NoSuchBucket.into_response(),
    };
    
    let file_path = format!("./data/{}", bucket);
    let path = StdPath::new(&file_path);
    
    if path.exists() {
        return ErrorCode::BucketAlreadyExists.into_response();
    }
    
    if let Err(e) = fs::create_dir_all(&path) {
        eprintln!("Failed to create bucket directory: {}", e);
        return ErrorCode::InternalError.into_response();
    }
    
    (StatusCode::CREATED).into_response()
}