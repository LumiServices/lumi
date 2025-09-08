use std::collections::HashMap;
use axum::{
    body::Bytes, extract::Path, http::{HeaderMap, HeaderValue, StatusCode}, response::IntoResponse
};
use serde::Serialize;
use tokio::fs;
use crate::{core::xml, s3::errors::ErrorCode};

#[derive(Serialize)]
#[serde(rename = "ListBucketResult")]
struct ListBucketResult {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Prefix", skip_serializing_if = "Option::is_none")]
    prefix: Option<String>,
    #[serde(rename = "IsTruncated")]
    is_truncated: bool,
    #[serde(rename = "Contents")]
    contents: Vec<S3Object>,
}

#[derive(Serialize)]
struct S3Object {
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "LastModified")]
    last_modified: String,
    #[serde(rename = "ETag")]
    etag: String,
    #[serde(rename = "Size")]
    size: u64,
    #[serde(rename = "StorageClass")]
    storage_class: String,
}

pub async fn put_object_handler(
    Path(params): Path<HashMap<String, String>>,
    headers: HeaderMap,
    body: Bytes
) -> impl IntoResponse {
    let bucket = match params.get("bucket") {
        Some(b) => b,
        None => return ErrorCode::NoSuchBucket.into_response(),
    };
    let key = match params.get("key") {
        Some(b) => b,
        None => return ErrorCode::InvalidRequest.into_response(),
    };
    let file_path = format!("./data/{}/{}", bucket, key); // <- I need to move this to some sort of config
    if let Err(e) = fs::create_dir_all(format!("./data/{}", bucket)).await {
        eprintln!("Directory creation error: {:?}", e);
        return ErrorCode::InternalError.into_response();
    }
    if let Some(content_type) = headers.get("content-type") {
        let content_type_path = format!("{}.content_type", file_path);
        if let Ok(ct_str) = content_type.to_str() {
            let _ = fs::write(&content_type_path, ct_str).await;
        }
    }
    match fs::write(&file_path, body).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => ErrorCode::NoSuchKey.into_response(),
            _ => ErrorCode::InternalError.into_response(),
        }
    }
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
    let content_type_path = format!("{}.content_type", file_path);
    match fs::read(&file_path).await {
       Ok(contents) => {
            let mut headers = HeaderMap::new();
            if let Ok(ct) = fs::read_to_string(&content_type_path).await {
                if let Ok(header_value) = HeaderValue::from_str(ct.trim()) {
                    headers.insert("content-type", header_value);
                } else {
                    headers.insert("content-type", HeaderValue::from_static("application/octet-stream"));
                }
            }
            (StatusCode::OK, headers, contents).into_response()
        },
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::NotFound => ErrorCode::NoSuchKey.into_response(),
                _ => ErrorCode::InternalError.into_response(),
            }
        }
    }
}

pub async fn list_objects_v2_handler(
    Path(params): Path<HashMap<String, String>>
) -> impl IntoResponse {
    let bucket = match params.get("bucket") {
        Some(b) => b,
        None => return ErrorCode::NoSuchBucket.into_response(),
    };
    let bucket_path = format!("./data/{}", bucket);
    let mut contents = Vec::new();
    match fs::read_dir(&bucket_path).await {
        Ok(mut entries) => {
            while let Some(entry) = entries.next_entry().await.unwrap_or(None) {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("content_type") {
                    continue;
                }
                if path.is_file() {
                    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                        match fs::metadata(&path).await {
                            Ok(metadata) => {
                                let size = metadata.len();
                                let timestamp = metadata.modified()
                                .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
                                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                                .unwrap_or_default()
                                .as_secs();
                                let last_modified = timestamp.to_string();
                                let etag = format!("\"{}\"", size);
                                contents.push(S3Object {
                                    key: file_name.to_string(),
                                    last_modified,
                                    etag,
                                    size,
                                    storage_class: "STANDARD".to_string(),
                                });
                            }
                            Err(_) => continue,
                        }
                    }
                }
            }
        }
        Err(e) => {
            return match e.kind() {
                std::io::ErrorKind::NotFound => ErrorCode::NoSuchBucket.into_response(),
                _ => ErrorCode::InternalError.into_response(),
            };
        }
    }
    let response = ListBucketResult {
        name: bucket.clone(),
        prefix: None,
        is_truncated: false,
        contents,
    };
    xml::XmlResponse(response).into_response()
}

pub async fn copy_object_handler(
    Path(params): Path<HashMap<String, String>>
) -> impl IntoResponse {
    
}

pub async fn delete_object_handler(
    Path(params): Path<HashMap<String, String>>
) -> impl IntoResponse {
    
}