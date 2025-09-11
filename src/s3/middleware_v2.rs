use axum::{
    extract::Request,
    http::{StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    body::Body,
};
use std::env;
use crate::s3::{
    errors::{ErrorCode, create_error_response},
    credentials::{DEFAULT_ACCESS_KEY, DEFAULT_SECRET_KEY, calculate_signature, parse_aws_credentials}
};
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

fn get_credentials_from_env() -> (String, String) {
    let access_key = env::var("lumi_access_key")
        .or_else(|_| env::var("lumi_access_key"))
        .unwrap_or_else(|_| DEFAULT_ACCESS_KEY.to_string());
    
    let secret_key = env::var("lumi_secret_key")
        .or_else(|_| env::var("lumi_secret_key"))
        .unwrap_or_else(|_| DEFAULT_SECRET_KEY.to_string());
    
    (access_key, secret_key)
}

fn authenticate_request(headers: &axum::http::HeaderMap, method: &str, uri: &str, body: &[u8]) -> ErrorCode {
    let creds = match parse_aws_credentials(headers) {
        Ok(creds) => creds,
        Err(e) => return e,
    };

    let (env_access_key, env_secret_key) = get_credentials_from_env();
    
    if creds.access_key != env_access_key {
    return ErrorCode::InvalidAccessKeyId;
        }

        let secret_key = &env_secret_key;
        

match calculate_signature(secret_key, method, uri, headers, body, &creds.credential_scope, &creds.signed_headers) {
        Ok(expected_signature) => {
            if creds.signature == expected_signature {
                ErrorCode::None
            } else {
                ErrorCode::SignatureDoesNotMatch
            }
        }
        Err(e) => e,
    }
}


pub async fn s3_auth_middleware(req: Request, next: Next) -> Response {
    let (parts, body) = req.into_parts();
    let method = parts.method.as_str();
    let uri = parts.uri.to_string();
    
    let body_bytes = match axum::body::to_bytes(body, usize::MAX).await {
        Ok(bytes) => bytes,
        Err(_) => return ErrorCode::InternalError.into_response(),
    };

    match authenticate_request(&parts.headers, method, &uri, &body_bytes) {
        ErrorCode::None => {
            let req = Request::from_parts(parts, Body::from(body_bytes));
            next.run(req).await
        },
        error => error.into_response(),
    }
}