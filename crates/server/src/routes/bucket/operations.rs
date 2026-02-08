use axum::{
    http::{Method, StatusCode},
    response::{
        IntoResponse,
        Response
    },
};
use std::collections::HashMap;
use tokio::fs::read_dir;
use chrono::{DateTime, Utc};
use lumi_utils::errors::ErrorCode;
use crate::xml::{
    ListAllMyBucketsResult, 
    Owner, 
    Buckets, 
    Bucket
};

pub async fn handle(
    method: Method,
    bucket: String,
    query: HashMap<String, String>,
) -> Response {
    match method {
        Method::GET => list_objects().await.into_response(),
        _ => ErrorCode::MethodNotAllowed.into_response(),
    }
}

pub async fn list_buckets() -> impl IntoResponse {
    let mut buckets = Vec::new();
    let mut entries = match read_dir("./data").await {
        Ok(e) => e,
        Err(_) => return ErrorCode::InternalError.into_response(),
    };
    while let Ok(Some(entry)) = entries.next_entry().await {
        if let Ok(filetype) = entry.file_type().await {
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
        owner: Owner::default(),
        buckets: Buckets { bucket: buckets },
    };
    match quick_xml::se::to_string(&response) {
        Ok(xml) => {
            let body = format!(r#"<?xml version="1.0" encoding="UTF-8"?>{}"#, xml);
            (StatusCode::OK, [(axum::http::header::CONTENT_TYPE, "application/xml")], body).into_response()
        }
        Err(_) => ErrorCode::InternalError.into_response()
    }
}

async fn list_objects() -> impl IntoResponse {
    ErrorCode::NotImplemented.into_response()
}