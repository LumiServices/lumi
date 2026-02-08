use axum::{
    body::{to_bytes, Body},
    extract::Request,
    http::Method,
    middleware::Next,
    response::{
        Response,
        IntoResponse
    },
};
use lumi_credentials::aws_sigv4;
use lumi_utils::errors::{ErrorCode, RestErrorResponse};

const REQUEST_ID: &str = "unknown";

fn s3_error(code: ErrorCode, resource: &str) -> Response {
    RestErrorResponse::from_error_code(code, resource.to_string(), REQUEST_ID.to_string())
        .into_response()
}

/// Public routes: no auth. Everything else is protected and must pass Sig V4.
fn is_public(method: &Method, path: &str) -> bool {
    let path = path.trim_matches('/');
    let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

    // GET /{bucket}/{key} — public object read
    if method == Method::GET && segments.len() >= 2 {
        return true;
    }
    false
}

fn require_auth(
    method: &str,
    uri: &str,
    headers: &axum::http::HeaderMap,
    body: &[u8],
    resource: &str,
    expected_access_key: &str,
    expected_secret_key: &str,
) -> Result<(), Response> {
    use aws_sigv4::ErrorCode as CredError;

    let creds = match aws_sigv4::parse_aws_credentials(headers) {
        Ok(c) => c,
        Err(CredError::MissingCredTag) => {
            return Err(s3_error(ErrorCode::MissingCredTag, resource));
        }
        Err(_) => {
            return Err(s3_error(ErrorCode::CredMalformed, resource));
        }
    };

    let secret = if creds.access_key == expected_access_key {
        expected_secret_key
    } else {
        return Err(s3_error(ErrorCode::InvalidAccessKeyId, resource));
    };

    let expected = match aws_sigv4::calculate_signature(
        secret,
        method,
        uri,
        headers,
        body,
        &creds.credential_scope,
        &creds.signed_headers,
    ) {
        Ok(s) => s,
        Err(_) => return Err(s3_error(ErrorCode::SignatureDoesNotMatch, resource)),
    };

    if expected != creds.signature {
        return Err(s3_error(ErrorCode::SignatureDoesNotMatch, resource));
    }

    Ok(())
}

pub async fn auth(
    request: Request,
    next: Next,
    expected_access_key: &str,
    expected_secret_key: &str,
) -> Response {
    if is_public(request.method(), request.uri().path()) {
        return next.run(request).await;
    }

    let (parts, body) = request.into_parts();
    let resource = parts.uri.path().to_string();
    let body_bytes = match to_bytes(body, usize::MAX).await {
        Ok(b) => b,
        Err(_) => return s3_error(ErrorCode::InvalidRequest, &resource),
    };

    let uri = parts.uri.to_string();
    let method = parts.method.as_str();
    if let Err(r) = require_auth(
        method,
        &uri,
        &parts.headers,
        &body_bytes,
        &resource,
        expected_access_key,
        expected_secret_key,
    ) {
        return r;
    }

    let request = Request::from_parts(parts, Body::from(body_bytes));
    next.run(request).await
}
