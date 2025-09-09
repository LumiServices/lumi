pub mod middleware;
pub mod middleware_v2;
pub mod credentials;
pub mod errors;
pub fn generate_request_id() -> String {
    //generate uuid and return itg
    return uuid::Uuid::new_v4().to_string();
}