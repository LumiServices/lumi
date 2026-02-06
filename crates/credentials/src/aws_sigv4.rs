// all this shit is backported from lumi v1 because im lazy as shit
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};
use axum::http::HeaderMap;
use std::collections::HashMap;
use crate::AWS_V4_ALGO;
type HmacSha256 = Hmac<Sha256>;
#[derive(Debug, Clone, PartialEq)]
pub struct AwsCredentials {
    pub access_key: String,
    pub credential_scope: String,
    pub signed_headers: String,
    pub signature: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorCode {
    CredMalformed,
    MissingCredTag,
    InvalidRequest,
    InternalError,
}

pub fn parse_aws_credentials(headers: &HeaderMap) -> Result<AwsCredentials, ErrorCode> {
    let auth_header = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .ok_or(ErrorCode::MissingCredTag)?;
    if !auth_header.starts_with(AWS_V4_ALGO) {
        return Err(ErrorCode::InvalidRequest);
    }
    let auth_parts = auth_header
        .strip_prefix(AWS_V4_ALGO)
        .unwrap()
        .trim();
    let parts = parse_auth_components(auth_parts)?;
    let credential = parts.get("Credential").ok_or(ErrorCode::CredMalformed)?;
    let signed_headers = parts.get("SignedHeaders").ok_or(ErrorCode::CredMalformed)?;
    let signature = parts.get("Signature").ok_or(ErrorCode::CredMalformed)?;
    let (access_key, credential_scope) = parse_credential_field(credential)?;
    Ok(AwsCredentials {
        access_key,
        credential_scope,
        signed_headers: signed_headers.to_string(),
        signature: signature.to_string(),
    })
}

fn parse_auth_components(auth_parts: &str) -> Result<HashMap<&str, &str>, ErrorCode> {
    let mut components = HashMap::new();
    for part in auth_parts.split(", ") {
        if let Some((key, value)) = part.split_once('=') {
            components.insert(key, value);
        }
    }
    Ok(components)
}

fn parse_credential_field(credential: &str) -> Result<(String, String), ErrorCode> {
    let mut parts = credential.splitn(2, '/');
    let access_key = parts
        .next()
        .ok_or(ErrorCode::CredMalformed)?
        .to_string();
    let credential_scope = parts
        .next()
        .ok_or(ErrorCode::CredMalformed)?
        .to_string();
    Ok((access_key, credential_scope))
}

pub fn calculate_signature(
    secret_key: &str,
    method: &str,
    uri: &str,
    headers: &HeaderMap,
    body: &[u8],
    credential_scope: &str,
    signed_headers: &str,
) -> Result<String, ErrorCode> {
    let canonical_request = get_canonical_request(
        method,
        uri,
        headers,
        body,
        signed_headers,
    )?;
    let string_to_sign = get_string_to_sign(
        &canonical_request,
        credential_scope,
        headers,
    )?;
    let signing_key = get_signing_key(secret_key, credential_scope)?;
    get_signature(signing_key, &string_to_sign)
}

fn get_canonical_request(
    method: &str,
    uri: &str,
    headers: &HeaderMap,
    body: &[u8],
    signed_headers: &str,
) -> Result<String, ErrorCode> {
    let method = method.to_uppercase();
    let (canonical_uri, canonical_query) = parse_uri_components(uri);
    let canonical_headers = get_canonical_headers(headers, signed_headers);
    let signed_headers_canonical = signed_headers.to_lowercase();
    let payload_hash = hex_encode(Sha256::digest(body));
    Ok(format!(
        "{}\n{}\n{}\n{}\n\n{}\n{}",
        method,
        canonical_uri,
        canonical_query,
        canonical_headers,
        signed_headers_canonical,
        payload_hash
    ))
}

fn parse_uri_components(uri: &str) -> (&str, String) {
    match uri.split_once('?') {
        Some((path, query)) if !query.is_empty() => {
            let mut params: Vec<&str> = query.split('&').collect();
            params.sort_unstable();
            (path, params.join("&"))
        }
        _ => (uri, String::new()),
    }
}

fn get_canonical_headers(headers: &HeaderMap, signed_headers: &str) -> String {
    signed_headers
        .split(';')
        .map(|name| {
            let name = name.trim().to_lowercase();
            let value = headers
                .get(&name)
                .and_then(|v| v.to_str().ok())
                .unwrap_or("")
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");
            format!("{}:{}", name, value)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn get_string_to_sign(
    canonical_request: &str,
    credential_scope: &str,
    headers: &HeaderMap,
) -> Result<String, ErrorCode> {
    let timestamp = headers
        .get("x-amz-date")
        .and_then(|v| v.to_str().ok())
        .ok_or(ErrorCode::CredMalformed)?;
    let request_hash = hex_encode(Sha256::digest(canonical_request.as_bytes()));
    Ok(format!(
        "{}\n{}\n{}\n{}",
        AWS_V4_ALGO,
        timestamp,
        credential_scope,
        request_hash
    ))
}

fn get_signing_key(secret_key: &str, credential_scope: &str) -> Result<Vec<u8>, ErrorCode> {
    let parts: Vec<&str> = credential_scope.split('/').collect();
    if parts.len() != 4 {
        return Err(ErrorCode::CredMalformed);
    }
    let date = parts[0];
    let region = parts[1];
    let service = parts[2];
    let k_secret = format!("AWS4{}", secret_key);
    let k_date = hmac_sha256(k_secret.as_bytes(), date.as_bytes())?;
    let k_region = hmac_sha256(&k_date, region.as_bytes())?;
    let k_service = hmac_sha256(&k_region, service.as_bytes())?;
    let k_signing = hmac_sha256(&k_service, b"aws4_request")?;
    Ok(k_signing)
}

fn hmac_sha256(key: &[u8], data: &[u8]) -> Result<Vec<u8>, ErrorCode> {
    let mut mac = HmacSha256::new_from_slice(key)
        .map_err(|_| ErrorCode::InternalError)?;
    mac.update(data);
    Ok(mac.finalize().into_bytes().to_vec())
}

fn get_signature(signing_key: Vec<u8>, string_to_sign: &str) -> Result<String, ErrorCode> {
    let mut mac = HmacSha256::new_from_slice(&signing_key)
        .map_err(|_| ErrorCode::InternalError)?;
    mac.update(string_to_sign.as_bytes());
    Ok(hex_encode(mac.finalize().into_bytes()))
}

fn hex_encode<T: AsRef<[u8]>>(data: T) -> String {
    hex::encode(data)
}
#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderMap;
    #[test]
    fn test_parse_credential_field() {
        let cred = "AKIAIOSFODNN7EXAMPLE/20130524/us-east-1/s3/aws4_request";
        let (access_key, scope) = parse_credential_field(cred).unwrap();
        assert_eq!(access_key, "AKIAIOSFODNN7EXAMPLE");
        assert_eq!(scope, "20130524/us-east-1/s3/aws4_request");
    }

    #[test]
    fn test_parse_uri_components_no_query() {
        let (path, query) = parse_uri_components("/bucket/key");
        assert_eq!(path, "/bucket/key");
        assert_eq!(query, "");
    }
    #[test]
    fn test_parse_uri_components_with_query() {
        let (path, query) = parse_uri_components("/bucket/key?prefix=test&max-keys=10");
        assert_eq!(path, "/bucket/key");
        assert_eq!(query, "max-keys=10&prefix=test");
    }
    #[test]
    fn test_signing_key_derivation() {
        let secret = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY";
        let scope = "20130524/us-east-1/s3/aws4_request";
        let key = get_signing_key(secret, scope).unwrap();
        assert_eq!(key.len(), 32);
    }
    #[test]
    fn test_parse_auth_header() {
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::AUTHORIZATION,
            "AWS4-HMAC-SHA256 Credential=AKIAIOSFODNN7EXAMPLE/20130524/us-east-1/s3/aws4_request, SignedHeaders=host;x-amz-date, Signature=abcdef123456"
                .parse()
                .unwrap(),
        );
        let creds = parse_aws_credentials(&headers).unwrap();
        assert_eq!(creds.access_key, "AKIAIOSFODNN7EXAMPLE");
        assert_eq!(creds.credential_scope, "20130524/us-east-1/s3/aws4_request");
        assert_eq!(creds.signed_headers, "host;x-amz-date");
        assert_eq!(creds.signature, "abcdef123456");
    }
    #[test]
    fn test_invalid_credentials() {
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::AUTHORIZATION,
            "AWS4-HMAC-SHA256 Credential=INVALIDKEY, SignedHeaders=host, Signature=sig"
                .parse()
                .unwrap(),
        );

        let result = parse_aws_credentials(&headers);
        assert_eq!(result, Err(ErrorCode::CredMalformed));
    }
}