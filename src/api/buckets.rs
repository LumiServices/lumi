use serde::Serialize;
use chrono::{DateTime, Utc};
use std::{collections::HashMap, fs};
use std::path::Path as StdPath;
use axum::{extract::Path, response::IntoResponse};
use reqwest::StatusCode;
use crate::core::xml;
use crate::s3::errors::ErrorCode;
use crate::s3::policies::{
    BucketPolicy,
    PolicyManager,
};

#[derive(Serialize)]
#[serde(rename = "ListAllMyBucketsResult", rename_all = "PascalCase")]
struct ListAllMyBucketsResult {
    owner: Owner,
    buckets: Buckets,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct Owner {
    id: String,
    display_name: String,
}

#[derive(Serialize)]
struct Buckets {
    #[serde(rename = "Bucket")]
    bucket: Vec<Bucket>,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct Bucket {
    name: String,
    creation_date: String,
}

pub async fn create_bucket_handler(
    Path(params): Path<HashMap<String, String>>,
) -> impl IntoResponse {
    let bucket = match params.get("bucket") {
        Some(b) => b,
        None => return ErrorCode::NoSuchBucket.into_response(),
    };
    if bucket == "hunter2" {
        return ErrorCode::Hunter2.into_response();
    } 
    let file_path = format!("./data/{}", bucket);
    let path = StdPath::new(&file_path);
    
    if path.exists() {
        return ErrorCode::BucketAlreadyExists.into_response();
    }
    if let Err(e) = fs::create_dir_all(&path) {
        eprintln!("Failed to create bucket directory: {}", e);
        return ErrorCode::InternalError.into_response();
    }
    if let Err(e) = PolicyManager::set_bucket_policy(bucket, BucketPolicy::PublicRead) {
    eprintln!("Failed to set default bucket policy: {}", e);
    } 
    (StatusCode::CREATED).into_response()
}

pub async fn delete_bucket_handler(
    Path(params): Path<HashMap<String, String>>,
) -> impl IntoResponse {
    let bucket = match params.get("bucket") {
        Some(b) => b,
        None => return ErrorCode::NoSuchBucket.into_response(),
    };
    let file_path = format!("./data/{}", bucket);
    let path = StdPath::new(&file_path);
    if !path.exists() {
        return ErrorCode::NoSuchBucket.into_response()
    }
    match fs::read_dir(&path) {
        Ok(mut entries) => {
            if entries.next().is_some() {
                return ErrorCode::BucketNotEmpty.into_response()
            }
        }
        Err(_) => return  ErrorCode::InternalError.into_response()
    }
    if let Err(e) = fs::remove_dir(&path) {
        eprintln!("Failed to delete bucket directory: {}", e);
        return ErrorCode::InternalError.into_response();
    }
    let db = crate::db::sqlite::DB.get().unwrap();
    let _ = db.delete("bucket_policies", "bucket_name", bucket.as_bytes());
    (StatusCode::CREATED).into_response()
}

pub async fn list_buckets_handler() -> impl IntoResponse {
    let mut buckets = Vec::new();
    let mut entries = match tokio::fs::read_dir("./data").await {
        Ok(e) => e,
        Err(_) => return ErrorCode::InternalError.into_response(),
    };
   while let Ok(Some(entry)) = entries.next_entry().await {
        if let Ok(filetype) = entry.file_type().await{
            if filetype.is_dir() {
                let metadata = entry.metadata().await.ok();
                let created = metadata
                    .and_then(|m| m.created().ok())
                    .map(|t| DateTime::<Utc>::from(t).to_rfc3339())
                    .unwrap_or_else(|| Utc::now().to_rfc3339());
                buckets.push(Bucket {
                    name: entry.file_name().to_string_lossy().to_string(),
                    creation_date: created,
                });
            }
        }
   } 
   let response = ListAllMyBucketsResult {
        owner: Owner {
            id: "0".into(),
            display_name: "lumi".into(),
        }, 
        buckets: Buckets { bucket: buckets },
    };
    xml::XmlResponse(response).into_response()
}