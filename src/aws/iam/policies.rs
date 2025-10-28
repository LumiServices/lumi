use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum Effect {
    Allow,
    Deny,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PolicyStatement {
    pub sid: Option<String>,
    pub effect: Effect,
    pub action: Vec<String>,
    pub resource: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Policy {
    pub version: String,
    pub statement: Vec<PolicyStatement>,
}

impl Policy {
    pub fn allows(&self, action: &str, resource: &str) -> bool {
        let mut allowed = false;
        for stmt in &self.statement {
            let actionmatch = stmt.action.iter().any(|a| a == action || a == "*");
            let resourcematch = stmt.resource.iter().any(|r| resource.starts_with(r) || r == "*");
            if actionmatch && resourcematch {
                match stmt.effect {
                    Effect::Deny => return false,
                    Effect::Allow => allowed = true,
                }
            }
        }
        allowed
    }
}