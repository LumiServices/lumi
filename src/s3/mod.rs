pub mod middleware;
pub mod credentials;
pub mod signature_parser;
pub mod errors;
pub fn generate_request_id() -> String {
    //generate uuid and return it
    return uuid::Uuid::new_v4().to_string();
}