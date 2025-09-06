//(I'm too stupid to find the docs again https://docs.aws.amazon.com/AmazonS3/latest/API/sig-v4-authenticating-requests.html)
use std::fmt;
use std::io::{self};
use base64::{engine::general_purpose, Engine as _};
use rand::RngCore;
pub const MIN_LEG_ACCESS_KEY: usize = 6;
pub const MAX_LEG_ACCESS_KEY: usize = 20;
pub const MIN_LEG_SECRET_KEY: usize = 6;
pub const MAX_LEG_SECRET_KEY: usize = 40;
pub const ALPHA_NUMERIC_TABLE: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const RESERVED_CHARS: &str = "=,";
//// Default credentials (if youre too stupid to setup your own)
pub const DEFAULT_ACCESS_KEY: &str = "lumiserver";
pub const DEFAULT_SECRET_KEY: &str = "lumiserver";

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
            KeyError::InvalidAccessKeyLength => {
                write!(
                    f,
                    "access key length should be between {} and {}",
                    MIN_LEG_ACCESS_KEY, MAX_LEG_ACCESS_KEY
                )
            }
            KeyError::InvalidSecretKeyLength => {
                write!(
                    f,
                    "secret key length should be between {} and {}",
                    MIN_LEG_SECRET_KEY, MAX_LEG_SECRET_KEY
                )
            }
            KeyError::NoAccessKeyWithSecretKey => {
                write!(f, "access key must be specified if secret key is specified")
            }
            KeyError::NoSecretKeyWithAccessKey => {
                write!(f, "secret key must be specified if access key is specified")
            }
            KeyError::ContainsReservedChars => {
                write!(f, "access key contains one of reserved characters '=' or ','")
            }
            KeyError::AccessKeyTooShort => {
                write!(f, "auth: access key length is too short")
            }
            KeyError::SecretKeyTooShort => {
                write!(f, "auth: secret key length is too short")
            }
            KeyError::IoError(msg) => {
                write!(f, "IO error: {}", msg)
            }
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
        Self {
            access_key,
            secret_key,
        }
    }
    pub fn default() -> Self {
        Self {
            access_key: DEFAULT_ACCESS_KEY.to_string(),
            secret_key: DEFAULT_SECRET_KEY.to_string(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorCode {
    ErrNone,
    ErrMissingCredTag,
    ErrInvalidAccessKeyID,
}
pub fn contains_reserved_chars(s: &str) -> bool {
    s.chars().any(|c| RESERVED_CHARS.contains(c))
}

pub fn generate_access_key(length: Option<usize>) -> Result<String, KeyError> {
    generate_access_key_with_reader(length, &mut rand::rng())
}

pub fn generate_access_key_with_reader<R: RngCore>(
    length: Option<usize>,
    rng: &mut R,
) -> Result<String, KeyError> {
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
pub fn generate_secret_key_with_reader<R: RngCore>(
    length: Option<usize>,
    rng: &mut R,
) -> Result<String, KeyError> {
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
pub fn validate_access_key(
    access_key: &str,
) -> (Credentials, bool, ErrorCode) {
    let cred = Credentials::new(access_key.to_string(), "".to_string());
    if cred.access_key.is_empty() {
        return (Credentials::default(), false, ErrorCode::ErrMissingCredTag);
    }
    if !access_key.is_empty() && cred.access_key != access_key {
        return (Credentials::default(), false, ErrorCode::ErrInvalidAccessKeyID);
    }
    (cred, true, ErrorCode::ErrNone)
}