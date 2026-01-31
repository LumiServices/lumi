use crate::{
    ALPHA_NUMERIC_TABLE, 
    MAX_LEG_ACCESS_KEY, 
    MAX_LEG_SECRET_KEY, 
    MIN_LEG_ACCESS_KEY, 
    MIN_LEG_SECRET_KEY
};

use rand::Rng;

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

impl std::fmt::Display for KeyError {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

pub fn generate_access_key(length: usize) -> Result<String, KeyError> {
        if length < MIN_LEG_ACCESS_KEY {
            return Err(KeyError::AccessKeyTooShort)
        }
        if length > MAX_LEG_ACCESS_KEY {
            return Err(KeyError::InvalidAccessKeyLength)
        }
        let mut result = String::with_capacity(length);
        let mut rng = rand::rng();
        for _ in 0..length {
            let idx = rng.random_range(0..ALPHA_NUMERIC_TABLE.len());
            result.push(ALPHA_NUMERIC_TABLE[idx] as char);
        }
        Ok(result)
}

// same shit pretty much :/
pub fn generate_secret_key(length: usize) -> Result<String, KeyError> {
        if length < MIN_LEG_SECRET_KEY {
            return Err(KeyError::SecretKeyTooShort)
        }
        if length > MAX_LEG_SECRET_KEY {
            return Err(KeyError::InvalidSecretKeyLength)
        }
        let mut result = String::with_capacity(length);
        let mut rng = rand::rng();
        for _ in 0..length {
            let idx = rng.random_range(0..ALPHA_NUMERIC_TABLE.len());
            result.push(ALPHA_NUMERIC_TABLE[idx] as char);
        }
        Ok(result)
}


#[derive(Debug, Clone, PartialEq)]
pub struct Credentials {
   pub access_key: String,
   pub secret_key: String,
}