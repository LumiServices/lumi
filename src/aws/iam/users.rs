use serde::{Deserialize, Serialize};
use crate::aws::iam::policies::Policy;
use crate::db::sqlite::DB;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub access_key: String,
    pub secret_key: String,
    pub policies: Vec<Policy>,
    pub created_at: String,
}

impl User {
    pub fn new(username: String, access_key: String, secret_key: String) -> Self {
        Self { username, access_key, secret_key, policies: Vec::new(), created_at: chrono::Utc::now().to_rfc3339() }
    }
    pub fn attach_policy(&mut self, policy: Policy) {
        self.policies.push(policy);
    }
    pub fn can_perform(&self, action: &str, resource: &str) -> bool {
        self.policies.iter().any(|p| p.allows(action, resource))
    }
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let db = DB.get().ok_or("Database not initialized")?;
        db.insert(
            "iam_users",
            "access_key",
            "user_data",
            self.access_key.as_bytes(),
        serde_json::to_string(self)?.as_bytes(),
        )?;
        Ok(())
    }
    pub fn get_user(access_key: &str) -> Result<Option<User>, Box<dyn std::error::Error>> {
        let db = DB.get().ok_or("Database not initialized")?;
        match db.get("iam_users", "access_key", "user_data", access_key.as_bytes())? {
            Some(data) => {
                let json = String::from_utf8(data)?;
                Ok(Some(serde_json::from_str(&json)?))
            }
            None => Ok(None),
        }
    }
}