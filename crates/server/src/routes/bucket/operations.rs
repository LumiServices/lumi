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
        Method::PUT => create_bucket(bucket).await.into_response(),
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
pub async fn create_bucket(bucket: String) -> impl IntoResponse {
    if bucket.len() < 3 || bucket.len() > 63
        || !bucket.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '.')
        || bucket.starts_with('-') || bucket.ends_with('-')
        || bucket.contains("..")
    {
        return ErrorCode::InvalidBucketName.into_response();
    }
        let path = format!("./data/{}", bucket);
        if bucket.is_empty() {
        return ErrorCode::NoSuchBucket.into_response();
    }
    /*
        <Cthon98> hey, if you type in your pw, it will show as stars
        <Cthon98> ********* see!
        <AzureDiamond> hunter2
        <AzureDiamond> doesnt look like stars to me
        <Cthon98> <AzureDiamond> *******
        <Cthon98> thats what I see
        <AzureDiamond> oh, really?
        <Cthon98> Absolutely
        <AzureDiamond> you can go hunter2 my hunter2-ing hunter2
        <AzureDiamond> haha, does that look funny to you?
        <Cthon98> lol, yes. See, when YOU type hunter2, it shows to us as *******
        <AzureDiamond> thats neat, I didnt know IRC did that
        <Cthon98> yep, no matter how many times you type hunter2, it will show to us as *******
        <AzureDiamond> awesome!
        <AzureDiamond> wait, how do you know my pw?
        <Cthon98> er, I just copy pasted YOUR ******'s and it appears to YOU as hunter2 cause its your pw
        <AzureDiamond> oh, ok.
     */
    if bucket == "hunter2" {
        return ErrorCode::Hunter2.into_response();
    }
    match tokio::fs::create_dir_all(&path).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => ErrorCode::BucketAlreadyExists.into_response(),
    }
}