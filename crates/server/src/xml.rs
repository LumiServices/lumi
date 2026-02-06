use serde::Serialize;
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Owner {
    pub id: String,
    pub display_name: String,
}
#[derive(Serialize)]
#[serde(rename = "ListAllMyBucketsResult", rename_all = "PascalCase")]
pub struct ListAllMyBucketsResult {
    pub owner: Owner,
    pub buckets: Buckets,
}
#[derive(Serialize)]
pub struct Buckets {
    #[serde(rename = "Bucket")]
    pub bucket: Vec<Bucket>,
}
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Bucket {
    pub name: String,
    pub creation_date: String,
}
impl Default for Owner {
    fn default() -> Self {
        Self {
            id: "lumiserver".to_string(),
            display_name: "lumiserver".to_string(),
        }
    }
}