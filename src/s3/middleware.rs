use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    body::Body,
};
use std::env;

use crate::s3::errors::{ErrorCode, create_error_response};
use crate::core::xml::xml_response;
use crate::s3::generate_request_id;
impl IntoResponse for ErrorCode {
    fn into_response(self) -> Response {
        if self == ErrorCode::None {
            return StatusCode::OK.into_response();
        }
        let request_id = generate_request_id();
        let error_response = create_error_response(self, "/".to_string(), request_id, None, None);
        let xml_body = xml_response(&error_response).unwrap_or_else(|_| {
            r#"<?xml version="1.0" encoding="UTF-8"?>
            <Error>
                <Code>InternalError</Code>
                <Message>Failed to serialize error</Message>
            </Error>"#.to_string()
        });
        Response::builder()
            .status(error_response.status_code)
            .header("Content-Type", "application/xml")
            .header("x-amz-request-id", &error_response.request_id)
            .body(Body::from(xml_body))
            .unwrap()
    }
}

fn authenticate_request(headers: &axum::http::HeaderMap) -> ErrorCode {
    let auth_header = match headers.get(header::AUTHORIZATION) {
        Some(header_value) => match header_value.to_str() {
            Ok(s) => s,
            Err(_) => return ErrorCode::CredMalformed,
        },
        None => return ErrorCode::MissingCredTag,
    };

    let access_key = match auth_header.split("Credential=").nth(1) {
        Some(cred_part) => match cred_part.split('/').next() {
            Some(key) => key,
            None => return ErrorCode::CredMalformed,
        },
        None => return ErrorCode::CredMalformed,
    };

    let expected_key = match env::var("lumi_access_key") {
        Ok(key) => key,
        Err(_) => return ErrorCode::AuthNotSetup,
    };

    if expected_key.is_empty() {
        return ErrorCode::AuthNotSetup;
    }

    if access_key != expected_key {
        return ErrorCode::InvalidAccessKeyId;
    }

    ErrorCode::None
}

pub async fn s3_auth_middleware(req: Request, next: Next) -> Response {
    match authenticate_request(req.headers()) {
        ErrorCode::None => next.run(req).await,
        error => error.into_response(),
    }
}