use quick_xml::se::to_string;
use serde::Serialize;
use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
pub struct XmlResponse<T: Serialize>(pub T);
impl<T: Serialize> IntoResponse for XmlResponse<T> {
    fn into_response(self) -> Response {
        match to_string(&self.0) {
            Ok(serialized) => {
                let declaration = r#"<?xml version="1.0" encoding="UTF-8"?>"#;
                let xml = format!("{}{}", declaration, serialized);
                (
                    StatusCode::OK,
                    [(header::CONTENT_TYPE, "application/xml")],
                    xml,
                ).into_response()
            }
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}