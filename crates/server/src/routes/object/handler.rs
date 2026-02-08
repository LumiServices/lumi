use axum::{
    body::Bytes,
    extract::{Path, Query},
    http::{HeaderMap, Method},
    response::{IntoResponse, Response},
};
use lumi_utils::errors::ErrorCode;
use std::collections::HashMap;
use super::operations;

pub async fn handler(
    method: Method,
    Path(params): Path<HashMap<String, String>>,
    Query(query): Query<HashMap<String, String>>,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    let bucket = match params.get("bucket") {
        Some(b) => b.clone(),
        None => return ErrorCode::NoSuchBucket.into_response(),
    };
    let key = match params.get("key") {
        Some(k) => k.clone(),
        None => return ErrorCode::InvalidRequest.into_response(),
    };
    operations::handle(method, bucket, key, headers, body).await.into_response()
}
