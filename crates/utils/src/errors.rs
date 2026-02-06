use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIError {
    pub code: String,
    pub description: String,
    pub http_status_code: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestErrorResponse {
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "Resource")]
    pub resource: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
    #[serde(rename = "Key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "BucketName", skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    #[serde(skip)]
    pub status_code: StatusCode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCode {
    None,
    AccessDenied,
    Hunter2,
    MethodNotAllowed,
    BucketNotEmpty,
    BucketAlreadyExists,
    BucketAlreadyOwnedByYou,
    NoSuchBucket,
    NoSuchBucketPolicy,
    NoSuchCorsConfiguration,
    NoSuchLifecycleConfiguration,
    NoSuchKey,
    NoSuchUpload,
    InvalidBucketName,
    InvalidDigest,
    InvalidMaxKeys,
    InvalidMaxUploads,
    InvalidMaxParts,
    InvalidMaxDeleteObjects,
    InvalidPartNumberMarker,
    InvalidPart,
    InvalidRange,
    InternalError,
    InvalidCopyDest,
    InvalidCopySource,
    InvalidTag,
    AuthHeaderEmpty,
    SignatureVersionNotSupported,
    MalformedPostRequest,
    PostFileRequired,
    PostPolicyConditionInvalidFormat,
    EntityTooSmall,
    EntityTooLarge,
    MissingFields,
    MissingCredTag,
    CredMalformed,
    MalformedXml,
    MalformedDate,
    MalformedPresignedDate,
    MalformedCredentialDate,
    MissingSignHeadersTag,
    MissingSignTag,
    UnsignedHeaders,
    InvalidQueryParams,
    InvalidQuerySignatureAlgo,
    ExpiredPresignRequest,
    MalformedExpires,
    NegativeExpires,
    MaximumExpires,
    SignatureDoesNotMatch,
    ContentSha256Mismatch,
    InvalidAccessKeyId,
    RequestNotReadyYet,
    MissingDateHeader,
    InvalidRequest,
    AuthNotSetup,
    NotImplemented,
    PreconditionFailed,
    ExistingObjectIsDirectory,
    ExistingObjectIsFile,
    TooManyRequest,
    RequestBytesExceed,
    OwnershipControlsNotFoundError,
    NoSuchTagSet,
}

impl ErrorCode {
    pub fn to_api_error(self) -> APIError {
        match self {
            ErrorCode::AccessDenied => APIError {
                code: "AccessDenied".into(),
                description: "Access Denied.".into(),
                http_status_code: 403,
            },
            ErrorCode::Hunter2 => APIError {
                code: "InvalidBucketName".into(),
                description: "The bucket name 'hunter2' appears as ******* to us.".into(),
                http_status_code: 418,
            },
            ErrorCode::MethodNotAllowed => APIError {
                code: "MethodNotAllowed".into(),
                description: "The specified method is not allowed against this resource.".into(),
                http_status_code: 405,
            },
            ErrorCode::BucketNotEmpty => APIError {
                code: "BucketNotEmpty".into(),
                description: "The bucket you tried to delete is not empty".into(),
                http_status_code: 409,
            },
            ErrorCode::BucketAlreadyExists => APIError {
                code: "BucketAlreadyExists".into(),
                description: "The requested bucket name is not available.".into(),
                http_status_code: 409,
            },
            ErrorCode::BucketAlreadyOwnedByYou => APIError {
                code: "BucketAlreadyOwnedByYou".into(),
                description: "Your previous request to create the named bucket succeeded and you already own it.".into(),
                http_status_code: 409,
            },
            ErrorCode::InvalidBucketName => APIError {
                code: "InvalidBucketName".into(),
                description: "The specified bucket is not valid.".into(),
                http_status_code: 400,
            },
            ErrorCode::NoSuchBucket => APIError {
                code: "NoSuchBucket".into(),
                description: "The specified bucket does not exist".into(),
                http_status_code: 404,
            },
            ErrorCode::NoSuchKey => APIError {
                code: "NoSuchKey".into(),
                description: "The specified key does not exist.".into(),
                http_status_code: 404,
            },
            ErrorCode::InternalError => APIError {
                code: "InternalError".into(),
                description: "We encountered an internal error, please try again.".into(),
                http_status_code: 500,
            },
            ErrorCode::NotImplemented => APIError {
                code: "NotImplemented".into(),
                description: "A header you provided implies functionality that is not implemented".into(),
                http_status_code: 501,
            },
            // Add more as needed, or use a default
            _ => APIError {
                code: "InternalError".into(),
                description: "An internal error occurred".into(),
                http_status_code: 500,
            },
        }
    }
}

impl IntoResponse for ErrorCode {
    fn into_response(self) -> Response {
        RestErrorResponse::from_error_code(self, "/".into(), "unknown".into()).into_response()
    }
}

impl IntoResponse for RestErrorResponse {
    fn into_response(self) -> Response {
        let status = self.status_code;
        
        match quick_xml::se::to_string(&self) {
            Ok(xml) => {
                let response = format!(r#"<?xml version="1.0" encoding="UTF-8"?>{}"#, xml);
                (status, [(header::CONTENT_TYPE, "application/xml")], response).into_response()
            }
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(header::CONTENT_TYPE, "application/xml")],
                r#"<?xml version="1.0" encoding="UTF-8"?><Error><Code>InternalError</Code></Error>"#,
            ).into_response()
        }
    }
}

impl RestErrorResponse {
    pub fn from_error_code(
        error_code: ErrorCode,
        resource: String,
        request_id: String,
    ) -> Self {
        let api_error = error_code.to_api_error();
        Self {
            code: api_error.code,
            message: api_error.description,
            resource,
            request_id,
            key: None,
            bucket_name: None,
            status_code: StatusCode::from_u16(api_error.http_status_code)
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    pub fn access_denied(request_id: String, resource: String) -> Self {
        Self::from_error_code(ErrorCode::AccessDenied, resource, request_id)
    }

    pub fn no_such_bucket(request_id: String, bucket_name: String) -> Self {
        let mut err = Self::from_error_code(
            ErrorCode::NoSuchBucket,
            format!("/{}", bucket_name),
            request_id,
        );
        err.bucket_name = Some(bucket_name);
        err
    }

    pub fn no_such_key(request_id: String, bucket_name: String, key: String) -> Self {
        let mut err = Self::from_error_code(
            ErrorCode::NoSuchKey,
            format!("/{}/{}", bucket_name, key),
            request_id,
        );
        err.key = Some(key);
        err.bucket_name = Some(bucket_name);
        err
    }

    pub fn internal_error(request_id: String) -> Self {
        Self::from_error_code(ErrorCode::InternalError, "/".into(), request_id)
    }
}