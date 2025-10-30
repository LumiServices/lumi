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
}

//policy manager