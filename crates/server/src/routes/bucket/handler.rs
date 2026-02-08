use axum::{
    body::Bytes,
    extract::{Path, Query},
    http::{HeaderMap, Method},
    response::{IntoResponse, Response},
};
use std::collections::HashMap;
use super::operations;

pub async fn handler(
    method: Method,
    path: Option<Path<HashMap<String, String>>>,
    Query(query): Query<HashMap<String, String>>,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    let params = match path {
        Some(p) => p.0,
        None => {return operations::list_buckets().await.into_response()},
    };
    let bucket = match params.get("bucket") {
        Some(b) if !b.is_empty() => b.clone(),
        _ => {return operations::list_buckets().await.into_response()},
    };
    operations::handle(method, bucket, query).await.into_response()
}
