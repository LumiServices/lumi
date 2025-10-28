use std::collections::HashMap;
use axum::{
    body::Body, extract::{Path, Request}, http::{HeaderMap, HeaderValue, StatusCode}, response::IntoResponse
};
use reqwest::Method;
use serde::Serialize;
use tokio::fs;
use crate::{core::xml, s3::errors::ErrorCode, db::sqlite::DB};
use chrono::{DateTime, Utc};

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
    body: Body
) -> impl IntoResponse {
    let bucket = match params.get("bucket") {
        Some(b) => b,
        None => return ErrorCode::NoSuchBucket.into_response(),
    };
    let key = match params.get("key") {
        Some(b) => b,
        None => return ErrorCode::InvalidRequest.into_response(),
    };
    if let Err(e) = fs::create_dir_all(format!("./data/{}", bucket)).await {
        eprintln!("Directory creation error: {:?}", e);
        return ErrorCode::InternalError.into_response();
    }
    if let Some(source_header_val) = headers.get("x-amz-copy-source") {
        let source_header = match source_header_val.to_str() {
            Ok(v) => v,
            Err(_) => return ErrorCode::InvalidRequest.into_response(),
        };
        if !source_header.is_empty() {
            let source = source_header.trim_start_matches("/");
            let parts: Vec<&str> = source.splitn(2, "/").collect();
            if parts.len() != 2 {
                return ErrorCode::InvalidRequest.into_response();
            }
            let source_bucket = parts[0];
            let source_key = parts[1];
            let source_file_path = format!("./data/{}/{}", source_bucket, source_key);
            let dest_file_path = format!("./data/{}/{}", bucket, key);
            if fs::metadata(&source_file_path).await.is_err() {
                return ErrorCode::NoSuchKey.into_response();
            }
            return match fs::copy(&source_file_path, &dest_file_path).await {
                Ok(_) => {
                    let db = DB.get().unwrap();
                    let source_object_key = format!("{}/{}", source_bucket, source_key);
                    let dest_object_key = format!("{}/{}", bucket, key);
                    let _ = db.delete("metadata", "object_key", dest_object_key.as_bytes());

                    if let Ok(Some(ct_bytes)) = db.get(
                        "metadata",
                        "object_key",
                        "content_type",
                        source_object_key.as_bytes(),
                    ) {
                        if let Err(e) = db.insert("metadata", "object_key", "content_type", dest_object_key.as_bytes(), &ct_bytes,)
                        {eprintln!("Failed to copy metadata: {:?}", e);}
                    }
                    StatusCode::OK.into_response()
                }
                Err(e) => match e.kind() {
                    std::io::ErrorKind::NotFound => ErrorCode::NoSuchKey.into_response(),
                    _ => ErrorCode::InternalError.into_response(),
                },
            };
        }
    }
    let body_bytes = match axum::body::to_bytes(body, usize::MAX).await {
        Ok(bytes) => bytes,
        Err(_) => return ErrorCode::InternalError.into_response(),
    };
    let file_path = format!("./data/{}/{}", bucket, key);
    match fs::write(&file_path, &body_bytes).await {
        Ok(_) => {
            if let Some(content_type) = headers.get("content-type") {
                if let Ok(ct_str) = content_type.to_str() {
                    let db = DB.get().unwrap();
                    let object_key = format!("{}/{}", bucket, key);
                    let _ = db.delete("metadata", "object_key", object_key.as_bytes());
                    if let Err(e) = db.insert(
                        "metadata",
                        "object_key",
                        "content_type",
                        object_key.as_bytes(),
                        ct_str.as_bytes(),
                    ) {
                        eprintln!("Failed to save content type to db: {:?}", e);
                    }
                }
            }
            StatusCode::OK.into_response()
        }
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => ErrorCode::NoSuchKey.into_response(),
            _ => ErrorCode::InternalError.into_response(),
        },
    }
}

pub async fn get_object_handler(
    Path(params): Path<HashMap<String, String>>,
    req: Request
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
    /*
    Hangin' with my cousin, readin' dirty magazines
    We seen two people kissin', we ain't know what that shit mean
    Then we start re-enactin' everything that we had seen
    That's when I gave my cousin HEAD, gave my cousin HEAD
    https://open.spotify.com/track/20fYUtTl5xpG8xXPZkU0yT?si=82a1aa416d0a4c02
    */
    if req.method() == Method::HEAD {
        match fs::metadata(&file_path).await {
            Ok(metadata) => {
                let mut headers = HeaderMap::new();
                headers.insert("content-length", 
            HeaderValue::from_str(&metadata.len().to_string()).unwrap());
                return (StatusCode::OK, headers).into_response();
            } 
            Err(_) => return ErrorCode::NoSuchKey.into_response(),
        }
    }
    match fs::read(&file_path).await {
       Ok(contents) => {
            let mut headers = HeaderMap::new();
            let db = DB.get().unwrap();
            let object_key = format!("{}/{}", bucket, key);
            
            match db.get("metadata", "object_key", "content_type", object_key.as_bytes()) {
                Ok(Some(ct_bytes)) => {
                    if let Ok(ct_str) = String::from_utf8(ct_bytes) {
                        if let Ok(header_value) = HeaderValue::from_str(&ct_str) {
                            headers.insert("content-type", header_value);
                        } else {
                            headers.insert("content-type", HeaderValue::from_static("application/octet-stream"));
                        }
                    }
                }
                _ => {
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
                                let system_time = metadata.modified()
                                    .unwrap_or(std::time::SystemTime::UNIX_EPOCH);
                                let datetime: DateTime<Utc> = system_time.into();
                                let last_modified = datetime.to_rfc3339();
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

pub async fn delete_object_handler(
    Path(params): Path<HashMap<String, String>>
) -> impl IntoResponse {
    let bucket = match params.get("bucket") {
        Some(b) => b,
        None => return ErrorCode::NoSuchBucket.into_response(),
    };
    let key = match params.get("key") {
        Some(b) => b,
        None => return ErrorCode::InvalidRequest.into_response(),
    };
    if !fs::metadata(format!("./data/{}", bucket)).await.is_ok() {
        return ErrorCode::NoSuchBucket.into_response();
    }
    
    match fs::remove_file(&format!("./data/{}/{}", bucket, key)).await {
        Ok(_) => {
            let db = DB.get().unwrap();
            let object_key = format!("{}/{}", bucket, key);
            let _ = db.delete("metadata", "object_key", object_key.as_bytes());
            StatusCode::NO_CONTENT.into_response()
        }
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::NotFound => {
                    StatusCode::NO_CONTENT.into_response()
                }
                _ => ErrorCode::InternalError.into_response(),
            }
        }
    }
}