use quick_xml::se::to_string;
use serde::{Serialize, Serializer};
use std::error::Error;

pub fn xml_response<T: Serialize>(payload: &T) -> Result<String, Box<dyn Error>> {
    let declaration = r#"<?xml version="1.0" encoding="UTF-8"?>"#;
    let serialized = to_string(payload)?;
    Ok(format!("{}{}", declaration, serialized))
}

pub fn aws_s3_xmlns<S: Serializer>(_: &(), ser: S) -> Result<S::Ok, S::Error> {
    ser.serialize_str("http://s3.amazonaws.com/doc/2006-03-01/")
}

pub fn w3_xsi_xmlns<S: Serializer>(_: &(), ser: S) -> Result<S::Ok, S::Error> {
    ser.serialize_str("http://www.w3.org/2001/XMLSchema-instance")
}

use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};

pub struct XmlResponse<T: Serialize>(pub T);

impl<T: Serialize> IntoResponse for XmlResponse<T> {
    fn into_response(self) -> Response {
        match xml_response(&self.0) {
            Ok(xml) => (
                StatusCode::OK,
                [(header::CONTENT_TYPE, "application/xml")],
                xml,
            ).into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}