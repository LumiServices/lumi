use std::fmt;
use std::io::{self};
use base64::{engine::general_purpose, Engine as _};
use axum::http::{HeaderMap, header};
use rand::RngCore;
use sha2::{Sha256, Digest};
use hmac::{Hmac, Mac};
use hex;
use crate::s3::errors::{ErrorCode};
type HmacSha256 = Hmac<Sha256>;
pub const MIN_LEG_ACCESS_KEY: usize = 6;
pub const MAX_LEG_ACCESS_KEY: usize = 20;
pub const MIN_LEG_SECRET_KEY: usize = 6;
pub const MAX_LEG_SECRET_KEY: usize = 40;
pub const ALPHA_NUMERIC_TABLE: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const RESERVED_CHARS: &str = "=,";
//default if youre stupid to setup your own
pub const DEFAULT_ACCESS_KEY: &str = "lumiserver";
pub const DEFAULT_SECRET_KEY: &str = "lumiserver";
const AWS_V4_ALGO: &str = "AWS4-HMAC-SHA256";

pub struct AwsCredentials {
   pub access_key: String,
   pub credential_scope: String, 
   pub signed_headers: String,
   pub signature: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum KeyError {
   InvalidAccessKeyLength,
   InvalidSecretKeyLength,
   NoAccessKeyWithSecretKey,
   NoSecretKeyWithAccessKey,
   ContainsReservedChars,
   AccessKeyTooShort,
   SecretKeyTooShort,
   IoError(String),
}

impl fmt::Display for KeyError {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       match self {
           KeyError::InvalidAccessKeyLength => write!(f, "access key length should be between {} and {}", MIN_LEG_ACCESS_KEY, MAX_LEG_ACCESS_KEY),
           KeyError::InvalidSecretKeyLength => write!(f, "secret key length should be between {} and {}", MIN_LEG_SECRET_KEY, MAX_LEG_SECRET_KEY),
           KeyError::NoAccessKeyWithSecretKey => write!(f, "access key must be specified if secret key is specified"),
           KeyError::NoSecretKeyWithAccessKey => write!(f, "secret key must be specified if access key is specified"),
           KeyError::ContainsReservedChars => write!(f, "access key contains one of reserved characters '=' or ','"),
           KeyError::AccessKeyTooShort => write!(f, "auth: access key length is too short"),
           KeyError::SecretKeyTooShort => write!(f, "auth: secret key length is too short"),
           KeyError::IoError(msg) => write!(f, "IO error: {}", msg),
       }
   }
}

impl std::error::Error for KeyError {}

impl From<io::Error> for KeyError {
   fn from(err: io::Error) -> Self {
       KeyError::IoError(err.to_string())
   }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Credentials {
   pub access_key: String,
   pub secret_key: String,
}

impl Credentials {
   pub fn new(access_key: String, secret_key: String) -> Self {
       Self { access_key, secret_key }
   }
   pub fn default() -> Self {
       Self { access_key: DEFAULT_ACCESS_KEY.to_string(), secret_key: DEFAULT_SECRET_KEY.to_string() }
   }
}

pub fn contains_reserved_chars(s: &str) -> bool {
   s.chars().any(|c| RESERVED_CHARS.contains(c))
}

pub fn generate_access_key(length: Option<usize>) -> Result<String, KeyError> {
   generate_access_key_with_reader(length, &mut rand::rng())
}

pub fn generate_access_key_with_reader<R: RngCore>(length: Option<usize>, rng: &mut R) -> Result<String, KeyError> {
   let length = length.unwrap_or(MAX_LEG_ACCESS_KEY);
   if length == 0 {
       return generate_access_key_with_reader(Some(MAX_LEG_ACCESS_KEY), rng);
   }
   if length < MIN_LEG_ACCESS_KEY {
       return Err(KeyError::AccessKeyTooShort);
   }
   let mut key = vec![0u8; length];
   rng.fill_bytes(&mut key);
   for byte in &mut key {
       *byte = ALPHA_NUMERIC_TABLE[(*byte as usize) % ALPHA_NUMERIC_TABLE.len()];
   }
   String::from_utf8(key).map_err(|_| KeyError::AccessKeyTooShort)
}

pub fn generate_secret_key(length: Option<usize>) -> Result<String, KeyError> {
   generate_secret_key_with_reader(length, &mut rand::rng())
}

pub fn generate_secret_key_with_reader<R: RngCore>(length: Option<usize>, rng: &mut R) -> Result<String, KeyError> {
   let length = length.unwrap_or(MAX_LEG_SECRET_KEY);
   if length == 0 {
       return generate_secret_key_with_reader(Some(MAX_LEG_SECRET_KEY), rng);
   }
   if length < MIN_LEG_SECRET_KEY {
       return Err(KeyError::SecretKeyTooShort);
   }
   let byte_length = (length * 3 + 3) / 4;
   let mut key = vec![0u8; byte_length];
   rng.fill_bytes(&mut key);
   let encoded = general_purpose::STANDARD_NO_PAD.encode(&key);
   let result = encoded.replace('/', "+");
   if result.len() > length {
       Ok(result[..length].to_string())
   } else {
       Ok(result)
   }
}

pub fn validate_access_key(access_key: &str) -> (Credentials, bool, ErrorCode) {
   let cred = Credentials::new(access_key.to_string(), "".to_string());
   if cred.access_key.is_empty() {
       return (Credentials::default(), false, ErrorCode::MissingCredTag);
   }
   if !access_key.is_empty() && cred.access_key != access_key {
       return (Credentials::default(), false, ErrorCode::InvalidAccessKeyId);
   }
   (cred, true, ErrorCode::None)
}

pub fn parse_aws_credentials(headers: &HeaderMap) -> Result<AwsCredentials, ErrorCode> {
   let auth_header = match headers.get(header::AUTHORIZATION) {
       Some(header_value) => match header_value.to_str() {
           Ok(s) => s,
           Err(_) => return Err(ErrorCode::CredMalformed),
       },
       None => return Err(ErrorCode::MissingCredTag),
   };
   let mut credential = None;
   let mut signed_headers = None;
   let mut signature = None;
   if !auth_header.starts_with(AWS_V4_ALGO) {
       return Err(ErrorCode::InvalidRequest);
   }
   let auth_parts = auth_header.strip_prefix(AWS_V4_ALGO).unwrap().trim();
   for part in auth_parts.split(", ") {
       if let Some(cred) = part.strip_prefix("Credential=") {
           credential = Some(cred);
       } else if let Some(headers) = part.strip_prefix("SignedHeaders=") {
           signed_headers = Some(headers);
       } else if let Some(sig) = part.strip_prefix("Signature=") {
           signature = Some(sig);
       }
   }
   let credential = credential.ok_or(ErrorCode::CredMalformed)?;
   let signed_headers = signed_headers.ok_or(ErrorCode::CredMalformed)?;
   let signature = signature.ok_or(ErrorCode::CredMalformed)?;
   let access_key = credential.split("/").next().ok_or(ErrorCode::CredMalformed)?.to_string();
   let credential_scope = credential.strip_prefix(&format!("{}/", access_key)).ok_or(ErrorCode::CredMalformed)?.to_string();
   Ok(AwsCredentials { access_key, credential_scope, signed_headers: signed_headers.to_string(), signature: signature.to_string() })
}

pub fn calculate_signature(secret_key: &str, method: &str, uri: &str, headers: &HeaderMap, body: &[u8], credential_scope: &str, signed_headers: &str) -> Result<String, ErrorCode> {
   let method = method.to_uppercase();
   let canonical_uri = uri.to_string();
   let canonical_query = "";
   let canonical_headers = signed_headers.split(';').map(|header_name| {
       let header_name = header_name.trim().to_lowercase();
       let header_value = headers.get(&header_name).and_then(|v| v.to_str().ok()).unwrap_or("").trim();
       format!("{}:{}", header_name, header_value)
   }).collect::<Vec<String>>().join("\n");
   let signed_headers_canonical = signed_headers.to_lowercase();
   let req_hash = hex::encode(Sha256::digest(body));
   let canonical_request = format!("{}\n{}\n{}\n{}\n\n{}\n{}", method, canonical_uri, canonical_query, canonical_headers, signed_headers_canonical, req_hash);
   let string_to_sign_str = string_to_sign(&canonical_request, credential_scope, headers)?;
   let signing_key = derive_signing_key(secret_key, credential_scope)?;
   let mut mac = HmacSha256::new_from_slice(&signing_key).map_err(|_| ErrorCode::InternalError)?;
   mac.update(string_to_sign_str.as_bytes());
   let signature = hex::encode(mac.finalize().into_bytes());
   Ok(signature)
}

fn string_to_sign(request: &str, credential_scope: &str, headers: &HeaderMap) -> Result<String, ErrorCode> {
   let string_to_sign = format!("{}\n{}\n{}\n{}", AWS_V4_ALGO, headers.get("x-amz-date").and_then(|v| v.to_str().ok()).ok_or(ErrorCode::CredMalformed)?, credential_scope, hex::encode(Sha256::digest(request.as_bytes())));
   Ok(string_to_sign)
}

fn derive_signing_key(secret_key: &str, credential_scope: &str) -> Result<Vec<u8>, ErrorCode> {
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
   let mut mac = HmacSha256::new_from_slice(key).map_err(|_| ErrorCode::InternalError)?;
   mac.update(data);
   Ok(mac.finalize().into_bytes().to_vec())
}