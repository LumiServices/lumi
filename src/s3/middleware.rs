use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use std::env;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ErrorCode {
    ErrNone,
    ErrMissingCredTag,
    ErrCredMalformed,
    ErrAuthNotSetup,
    ErrInvalidAccessKeyID,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: &'static str,
}

impl IntoResponse for ErrorCode {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ErrorCode::ErrNone => return StatusCode::OK.into_response(),
            ErrorCode::ErrMissingCredTag => (StatusCode::UNAUTHORIZED, "Missing Authorization header"),
            ErrorCode::ErrCredMalformed => (StatusCode::BAD_REQUEST, "Malformed credentials"),
            ErrorCode::ErrAuthNotSetup => (StatusCode::INTERNAL_SERVER_ERROR, "Authentication not configured"),
            ErrorCode::ErrInvalidAccessKeyID => (StatusCode::FORBIDDEN, "Invalid access key"),
        };

        (status, Json(ErrorResponse { error: message })).into_response()
    }
}

fn authenticate_request(headers: &axum::http::HeaderMap) -> ErrorCode {
    let auth_head = match headers.get(header::AUTHORIZATION) {
        Some(header_value) => match header_value.to_str() {
            Ok(s) => s,
            Err(_) => return ErrorCode::ErrCredMalformed,
        },
        None => return ErrorCode::ErrMissingCredTag,
    };

    let credential_parts: Vec<&str> = auth_head.split("Credential=").collect();
    if credential_parts.len() < 2 {
        return ErrorCode::ErrCredMalformed;
    }

    let access_key_parts: Vec<&str> = credential_parts[1].split('/').collect();
    if access_key_parts.is_empty() {
        return ErrorCode::ErrCredMalformed;
    }

    let access_key = access_key_parts[0];

    let expected_key = match env::var("lumi_access_key") {
        Ok(key) => key,
        Err(_) => return ErrorCode::ErrAuthNotSetup,
    };

    if expected_key.is_empty() {
        return ErrorCode::ErrAuthNotSetup;
    }

    if access_key != expected_key {
        return ErrorCode::ErrInvalidAccessKeyID;
    }

    ErrorCode::ErrNone
}

pub async fn s3_auth_middleware(req: Request, next: Next) -> Response {
    match authenticate_request(req.headers()) {
        ErrorCode::ErrNone => next.run(req).await,
        error => error.into_response(),
    }
}