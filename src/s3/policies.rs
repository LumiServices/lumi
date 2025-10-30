use serde::{Deserialize, Serialize};
use crate::db::sqlite::DB;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BucketPolicy {
    Private,
    PublicRead,
    Public,
}

impl Default for BucketPolicy {
    fn default() -> Self {
        BucketPolicy::PublicRead
    }
}

impl BucketPolicy {
    pub fn allows_anonymous_read(&self) -> bool {
        matches!(self, BucketPolicy::PublicRead | BucketPolicy::Public)
    }

    pub fn allows_anonymous_write(&self) -> bool {
        matches!(self, BucketPolicy::Public)
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            BucketPolicy::Private => "private",
            BucketPolicy::PublicRead => "public-read",
            BucketPolicy::Public => "public",
        }
    }
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "private" => Some(BucketPolicy::Private),
            "public-read" => Some(BucketPolicy::PublicRead),
            "public" => Some(BucketPolicy::Public),
            _ => None,
        }
    }
}

pub struct PolicyManager;

impl PolicyManager {
    pub fn get_bucket_policy(bucket_name: &str) -> Result<BucketPolicy, String> {
        let db = DB.get().ok_or("Database not initialized")?;
        match db.get("bucket_policies", "bucket_name", "policy", bucket_name.as_bytes()) {
                Ok(Some(policy_bytes)) => {
                    let policy_str = String::from_utf8(policy_bytes)
                    .map_err(|_| "Invalid policy data")?;
                    BucketPolicy::from_str(&policy_str).ok_or_else(|| format!("Unknown policy: {}", policy_str))
                }
                Ok(None) => Ok(BucketPolicy::default()),
                Err(e) => Err(format!("Database error: {}", e)),
        }
    }
    pub fn set_bucket_policy(bucket: &str, policy: BucketPolicy) -> Result<(), String> {
        let db = DB.get().ok_or("Database not initialized")?;
        let _ = db.delete("bucket_policies", "bucket_name", bucket.as_bytes());
        db.insert("bucket_policies", "bucket_name", "policy", bucket.as_bytes(), policy.as_str().as_bytes()).map_err(|e| format!("Failed to set policy: {}", e))
    }
}