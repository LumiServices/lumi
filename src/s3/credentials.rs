use axum::http::HeaderMap;
use crate::s3::api_errors::ErrorCode;

pub fn calculate_signature(
    secret_key: &str,
    method: &str,
    uri: &str,
    headers: HeaderMap,
    body: &[u8], 
    credential_scope: &str, 
    signed_headers: &str
) -> Result<String, ErrorCode>
{
}

// CAN I PUT MAY BALLS IN YO JAWS 
