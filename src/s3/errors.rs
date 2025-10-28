use axum::{
    http::StatusCode,
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

lazy_static::lazy_static! {
    static ref S3_ERROR_RESPONSE_MAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("AccessDenied", "Access Denied.");
        m.insert("BadDigest", "The Content-Md5 you specified did not match what we received.");
        m.insert("EntityTooSmall", "Your proposed upload is smaller than the minimum allowed object size.");
        m.insert("EntityTooLarge", "Your proposed upload exceeds the maximum allowed object size.");
        m.insert("IncompleteBody", "You did not provide the number of bytes specified by the Content-Length HTTP header.");
        m.insert("InternalError", "We encountered an internal error, please try again.");
        m.insert("InvalidAccessKeyId", "The access key ID you provided does not exist in our records.");
        m.insert("InvalidBucketName", "The specified bucket is not valid.");
        m.insert("InvalidDigest", "The Content-Md5 you specified is not valid.");
        m.insert("InvalidRange", "The requested range is not satisfiable");
        m.insert("MalformedXML", "The XML you provided was not well-formed or did not validate against our published schema.");
        m.insert("MissingContentLength", "You must provide the Content-Length HTTP header.");
        m.insert("MissingContentMD5", "Missing required header for this request: Content-Md5.");
        m.insert("MissingRequestBodyError", "Request body is empty.");
        m.insert("NoSuchBucket", "The specified bucket does not exist.");
        m.insert("NoSuchBucketPolicy", "The bucket policy does not exist");
        m.insert("NoSuchKey", "The specified key does not exist.");
        m.insert("NoSuchUpload", "The specified multipart upload does not exist. The upload ID may be invalid, or the upload may have been aborted or completed.");
        m.insert("NotImplemented", "A header you provided implies functionality that is not implemented");
        m.insert("PreconditionFailed", "At least one of the pre-conditions you specified did not hold");
        m.insert("RequestTimeTooSkewed", "The difference between the request time and the server's time is too large.");
        m.insert("SignatureDoesNotMatch", "The request signature we calculated does not match the signature you provided. Check your key and signing method.");
        m.insert("MethodNotAllowed", "The specified method is not allowed against this resource.");
        m.insert("InvalidPart", "One or more of the specified parts could not be found.");
        m.insert("InvalidPartOrder", "The list of parts was not in ascending order. The parts list must be specified in order by part number.");
        m.insert("InvalidObjectState", "The operation is not valid for the current state of the object.");
        m.insert("AuthorizationHeaderMalformed", "The authorization header is malformed; the region is wrong.");
        m.insert("MalformedPOSTRequest", "The body of your POST request is not well-formed multipart/form-data.");
        m.insert("BucketNotEmpty", "The bucket you tried to delete is not empty");
        m.insert("AllAccessDisabled", "All access to this bucket has been disabled.");
        m.insert("MalformedPolicy", "Policy has invalid resource.");
        m.insert("MissingFields", "Missing fields in request.");
        m.insert("AuthorizationQueryParametersError", "Error parsing the X-Amz-Credential parameter; the Credential is mal-formed; expecting \"<YOUR-AKID>/YYYYMMDD/REGION/SERVICE/aws4_request\".");
        m.insert("MalformedDate", "Invalid date format header, expected to be in ISO8601, RFC1123 or RFC1123Z time format.");
        m.insert("BucketAlreadyOwnedByYou", "Your previous request to create the named bucket succeeded and you already own it.");
        m.insert("InvalidDuration", "Duration provided in the request is invalid.");
        m.insert("XAmzContentSHA256Mismatch", "The provided 'x-amz-content-sha256' header does not match what was computed.");
        m.insert("NoSuchCORSConfiguration", "The CORS configuration does not exist");
        m
    };
    
    static ref ERROR_CODE_RESPONSE: HashMap<ErrorCode, APIError> = {
        let mut m = HashMap::new();
        
        m.insert(ErrorCode::AccessDenied, APIError {
            code: "AccessDenied".to_string(),
            description: "Access Denied.".to_string(),
            http_status_code: 403,
        });
        m.insert(ErrorCode::Hunter2, APIError {
            code: "InvalidBucketName".to_string(),
            description: "The bucket name 'hunter2' appears as ******* to us.".to_string(),
            http_status_code: 418,
        });
        m.insert(ErrorCode::MethodNotAllowed, APIError {
            code: "MethodNotAllowed".to_string(),
            description: "The specified method is not allowed against this resource.".to_string(),
            http_status_code: 405,
        });
        
        m.insert(ErrorCode::BucketNotEmpty, APIError {
            code: "BucketNotEmpty".to_string(),
            description: "The bucket you tried to delete is not empty".to_string(),
            http_status_code: 409,
        });
        
        m.insert(ErrorCode::BucketAlreadyExists, APIError {
            code: "BucketAlreadyExists".to_string(),
            description: "The requested bucket name is not available. The bucket name can not be an existing collection, and the bucket namespace is shared by all users of the system. Please select a different name and try again.".to_string(),
            http_status_code: 409,
        });
        
        m.insert(ErrorCode::BucketAlreadyOwnedByYou, APIError {
            code: "BucketAlreadyOwnedByYou".to_string(),
            description: "Your previous request to create the named bucket succeeded and you already own it.".to_string(),
            http_status_code: 409,
        });
        
        m.insert(ErrorCode::InvalidBucketName, APIError {
            code: "InvalidBucketName".to_string(),
            description: "The specified bucket is not valid.".to_string(),
            http_status_code: 400,
        });
        
        m.insert(ErrorCode::InvalidDigest, APIError {
            code: "InvalidDigest".to_string(),
            description: "The Content-Md5 you specified is not valid.".to_string(),
            http_status_code: 400,
        });
        
        m.insert(ErrorCode::InvalidMaxUploads, APIError {
            code: "InvalidArgument".to_string(),
            description: "Argument max-uploads must be an integer between 0 and 2147483647".to_string(),
            http_status_code: 400,
        });
        
        m.insert(ErrorCode::InvalidMaxKeys, APIError {
            code: "InvalidArgument".to_string(),
            description: "Argument maxKeys must be an integer between 0 and 2147483647".to_string(),
            http_status_code: 400,
        });
        
        m.insert(ErrorCode::InvalidMaxParts, APIError {
            code: "InvalidArgument".to_string(),
            description: "Argument max-parts must be an integer between 0 and 2147483647".to_string(),
            http_status_code: 400,
        });
        
        m.insert(ErrorCode::InvalidMaxDeleteObjects, APIError {
            code: "InvalidArgument".to_string(),
            description: "Argument objects can contain a list of up to 1000 keys".to_string(),
            http_status_code: 400,
        });
        
        m.insert(ErrorCode::InvalidPartNumberMarker, APIError {
            code: "InvalidArgument".to_string(),
            description: "Argument partNumberMarker must be an integer.".to_string(),
            http_status_code: 400,
        });
        
        m.insert(ErrorCode::NoSuchBucket, APIError {
            code: "NoSuchBucket".to_string(),
            description: "The specified bucket does not exist".to_string(),
            http_status_code: 404,
        });
        
        m.insert(ErrorCode::NoSuchBucketPolicy, APIError {
            code: "NoSuchBucketPolicy".to_string(),
            description: "The bucket policy does not exist".to_string(),
            http_status_code: 404,
        });
        
        m.insert(ErrorCode::NoSuchTagSet, APIError {
            code: "NoSuchTagSet".to_string(),
            description: "The TagSet does not exist".to_string(),
            http_status_code: 404,
        });
        
        m.insert(ErrorCode::NoSuchCorsConfiguration, APIError {
            code: "NoSuchCORSConfiguration".to_string(),
            description: "The CORS configuration does not exist".to_string(),
            http_status_code: 404,
        });
        
        m.insert(ErrorCode::NoSuchLifecycleConfiguration, APIError {
            code: "NoSuchLifecycleConfiguration".to_string(),
            description: "The lifecycle configuration does not exist".to_string(),
            http_status_code: 404,
        });
        
        m.insert(ErrorCode::NoSuchKey, APIError {
            code: "NoSuchKey".to_string(),
            description: "The specified key does not exist.".to_string(),
            http_status_code: 404,
        });
        
        m.insert(ErrorCode::NoSuchUpload, APIError {
            code: "NoSuchUpload".to_string(),
            description: "The specified multipart upload does not exist. The upload ID may be invalid, or the upload may have been aborted or completed.".to_string(),
            http_status_code: 404,
        });
        
        m.insert(ErrorCode::InternalError, APIError {
            code: "InternalError".to_string(),
            description: "We encountered an internal error, please try again.".to_string(),
            http_status_code: 500,
        });
        
        m.insert(ErrorCode::InvalidPart, APIError {
            code: "InvalidPart".to_string(),
            description: "One or more of the specified parts could not be found.  The part may not have been uploaded, or the specified entity tag may not match the part's entity tag.".to_string(),
            http_status_code: 400,
        });
        
        m.insert(ErrorCode::InvalidCopyDest, APIError {
            code: "InvalidRequest".to_string(),
            description: "This copy request is illegal because it is trying to copy an object to itself without changing the object's metadata, storage class, website redirect location or encryption attributes.".to_string(),
            http_status_code: 400,
        });
        
        m.insert(ErrorCode::InvalidCopySource, APIError {
            code: "InvalidArgument".to_string(),
            description: "Copy Source must mention the source bucket and key: sourcebucket/sourcekey.".to_string(),
            http_status_code: 400,
        });
        
        m.insert(ErrorCode::InvalidTag, APIError {
            code: "InvalidTag".to_string(),
            description: "The Tag value you have provided is invalid".to_string(),
            http_status_code: 400,
        });
        
        m.insert(ErrorCode::MalformedXml, APIError {
            code: "MalformedXML".to_string(),
            description: "The XML you provided was not well-formed or did not validate against our published schema.".to_string(),
            http_status_code: 400,
        });
        
        m.insert(ErrorCode::AuthHeaderEmpty, APIError {
            code: "InvalidArgument".to_string(),
            description: "Authorization header is invalid -- one and only one ' ' (space) required.".to_string(),
            http_status_code: 400,
        });
        
        m.insert(ErrorCode::SignatureVersionNotSupported, APIError {
            code: "InvalidRequest".to_string(),
            description: "The authorization mechanism you have provided is not supported. Please use AWS4-HMAC-SHA256.".to_string(),
            http_status_code: 400,
        });
        
        m.insert(ErrorCode::MalformedPostRequest, APIError {
            code: "MalformedPOSTRequest".to_string(),
            description: "The body of your POST request is not well-formed multipart/form-data.".to_string(),
            http_status_code: 400,
        });
        
        m.insert(ErrorCode::PostFileRequired, APIError {
            code: "InvalidArgument".to_string(),
            description: "POST requires exactly one file upload per request.".to_string(),
            http_status_code: 400,
        });
        
        m.insert(ErrorCode::PostPolicyConditionInvalidFormat, APIError {
            code: "PostPolicyInvalidKeyName".to_string(),
            description: "Invalid according to Policy: Policy Condition failed".to_string(),
            http_status_code: 403,
        });
        
        m.insert(ErrorCode::EntityTooSmall, APIError {
            code: "EntityTooSmall".to_string(),
            description: "Your proposed upload is smaller than the minimum allowed object size.".to_string(),
            http_status_code: 400,
        });
        
        m.insert(ErrorCode::EntityTooLarge, APIError {
            code: "EntityTooLarge".to_string(),
            description: "Your proposed upload exceeds the maximum allowed object size.".to_string(),
            http_status_code: 400,
        });
        
        m.insert(ErrorCode::MissingFields, APIError {
            code: "MissingFields".to_string(),
            description: "Missing fields in request.".to_string(),
            http_status_code: 400,
        });
        
        m.insert(ErrorCode::MissingCredTag, APIError {
            code: "InvalidRequest".to_string(),
            description: "Missing Credential field for this request.".to_string(),
            http_status_code: 400,
        });
        
        m.insert(ErrorCode::CredMalformed, APIError {
            code: "AuthorizationQueryParametersError".to_string(),
            description: "Error parsing the X-Amz-Credential parameter; the Credential is mal-formed; expecting \"<YOUR-AKID>/YYYYMMDD/REGION/SERVICE/aws4_request\".".to_string(),
            http_status_code: 400,
        });
        
        m.insert(ErrorCode::MalformedDate, APIError {
            code: "MalformedDate".to_string(),
            description: "Invalid date format header, expected to be in ISO8601, RFC1123 or RFC1123Z time format.".to_string(),
            http_status_code: 400,
        });
        
        m.insert(ErrorCode::MalformedPresignedDate, APIError {
            code: "AuthorizationQueryParametersError".to_string(),
            description: "X-Amz-Date must be in the ISO8601 Long Format \"yyyyMMdd'T'HHmmss'Z'\"".to_string(),
            http_status_code: 400,
        });
        
        m.insert(ErrorCode::MissingSignHeadersTag, APIError {
            code: "InvalidArgument".to_string(),
            description: "Signature header missing SignedHeaders field.".to_string(),
            http_status_code: 400,
        });
        
        m.insert(ErrorCode::MissingSignTag, APIError {
            code: "AccessDenied".to_string(),
            description: "Signature header missing Signature field.".to_string(),
            http_status_code: 400,
        });
        
        m.insert(ErrorCode::UnsignedHeaders, APIError {
            code: "AccessDenied".to_string(),
            description: "There were headers present in the request which were not signed".to_string(),
            http_status_code: 400,
        });
        
        m.insert(ErrorCode::InvalidQueryParams, APIError {
            code: "AuthorizationQueryParametersError".to_string(),
            description: "Query-string authentication version 4 requires the X-Amz-Algorithm, X-Amz-Credential, X-Amz-Signature, X-Amz-Date, X-Amz-SignedHeaders, and X-Amz-Expires parameters.".to_string(),
            http_status_code: 400,
        });
        
        m.insert(ErrorCode::InvalidQuerySignatureAlgo, APIError {
            code: "AuthorizationQueryParametersError".to_string(),
            description: "X-Amz-Algorithm only supports \"AWS4-HMAC-SHA256\".".to_string(),
            http_status_code: 400,
        });
        
        m.insert(ErrorCode::ExpiredPresignRequest, APIError {
            code: "AccessDenied".to_string(),
            description: "Request has expired".to_string(),
            http_status_code: 403,
        });
        
        m.insert(ErrorCode::MalformedExpires, APIError {
            code: "AuthorizationQueryParametersError".to_string(),
            description: "X-Amz-Expires should be a number".to_string(),
            http_status_code: 400,
        });
        
        m.insert(ErrorCode::NegativeExpires, APIError {
            code: "AuthorizationQueryParametersError".to_string(),
            description: "X-Amz-Expires must be non-negative".to_string(),
            http_status_code: 400,
        });
        
        m.insert(ErrorCode::MaximumExpires, APIError {
            code: "AuthorizationQueryParametersError".to_string(),
            description: "X-Amz-Expires must be less than a week (in seconds); that is, the given X-Amz-Expires must be less than 604800 seconds".to_string(),
            http_status_code: 400,
        });
        
        m.insert(ErrorCode::InvalidAccessKeyId, APIError {
            code: "InvalidAccessKeyId".to_string(),
            description: "The access key ID you provided does not exist in our records.".to_string(),
            http_status_code: 403,
        });
        
        m.insert(ErrorCode::RequestNotReadyYet, APIError {
            code: "AccessDenied".to_string(),
            description: "Request is not valid yet".to_string(),
            http_status_code: 403,
        });
        
        m.insert(ErrorCode::SignatureDoesNotMatch, APIError {
            code: "SignatureDoesNotMatch".to_string(),
            description: "The request signature we calculated does not match the signature you provided. Check your key and signing method.".to_string(),
            http_status_code: 403,
        });
        
        m.insert(ErrorCode::ContentSha256Mismatch, APIError {
            code: "XAmzContentSHA256Mismatch".to_string(),
            description: "The provided 'x-amz-content-sha256' header does not match what was computed.".to_string(),
            http_status_code: 400,
        });
        
        m.insert(ErrorCode::MissingDateHeader, APIError {
            code: "AccessDenied".to_string(),
            description: "AWS authentication requires a valid Date or x-amz-date header".to_string(),
            http_status_code: 400,
        });
        
        m.insert(ErrorCode::InvalidRequest, APIError {
            code: "InvalidRequest".to_string(),
            description: "Invalid Request".to_string(),
            http_status_code: 400,
        });
        
        m.insert(ErrorCode::InvalidRange, APIError {
            code: "InvalidRange".to_string(),
            description: "The requested range is not satisfiable".to_string(),
            http_status_code: 416,
        });
        
        m.insert(ErrorCode::AuthNotSetup, APIError {
            code: "InvalidRequest".to_string(),
            description: "Signed request requires setting up lumi authentication".to_string(),
            http_status_code: 400,
        });
        
        m.insert(ErrorCode::NotImplemented, APIError {
            code: "NotImplemented".to_string(),
            description: "A header you provided implies functionality that is not implemented".to_string(),
            http_status_code: 501,
        });
        
        m.insert(ErrorCode::PreconditionFailed, APIError {
            code: "PreconditionFailed".to_string(),
            description: "At least one of the pre-conditions you specified did not hold".to_string(),
            http_status_code: 412,
        });
        
        m.insert(ErrorCode::ExistingObjectIsDirectory, APIError {
            code: "ExistingObjectIsDirectory".to_string(),
            description: "Existing Object is a directory.".to_string(),
            http_status_code: 409,
        });
        
        m.insert(ErrorCode::ExistingObjectIsFile, APIError {
            code: "ExistingObjectIsFile".to_string(),
            description: "Existing Object is a file.".to_string(),
            http_status_code: 409,
        });
        
        m.insert(ErrorCode::TooManyRequest, APIError {
            code: "ErrTooManyRequest".to_string(),
            description: "Too many simultaneous request count".to_string(),
            http_status_code: 429,
        });
        
        m.insert(ErrorCode::RequestBytesExceed, APIError {
            code: "ErrRequestBytesExceed".to_string(),
            description: "Simultaneous request bytes exceed limitations".to_string(),
            http_status_code: 429,
        });
        
        m.insert(ErrorCode::OwnershipControlsNotFoundError, APIError {
            code: "OwnershipControlsNotFoundError".to_string(),
            description: "The bucket ownership controls were not found".to_string(),
            http_status_code: 404,
        });
        
        m
    };
}
pub fn get_api_error(code: ErrorCode) -> Option<&'static APIError> {
    ERROR_CODE_RESPONSE.get(&code)
}

pub fn create_error_response(
    error_code: ErrorCode,
    resource: String,
    request_id: String,
    key: Option<String>,
    bucket_name: Option<String>,
) -> RestErrorResponse {
    let default_error = APIError {
        code: "InternalError".to_string(),
        description: "An internal error occurred".to_string(),
        http_status_code: 500,
    };
    
    let api_error = get_api_error(error_code).unwrap_or(&default_error);

    RestErrorResponse {
        code: api_error.code.clone(),
        message: api_error.description.clone(),
        resource,
        request_id,
        key,
        bucket_name,
        status_code: StatusCode::from_u16(api_error.http_status_code)
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
impl RestErrorResponse {
    pub fn access_denied(request_id: String, resource: String) -> Self {
        create_error_response(ErrorCode::AccessDenied, resource, request_id, None, None)
    }

    pub fn no_such_bucket(request_id: String, bucket_name: String) -> Self {
        create_error_response(
            ErrorCode::NoSuchBucket,
            format!("/{}", bucket_name),
            request_id,
            None,
            Some(bucket_name),
        )
    }

    pub fn no_such_key(request_id: String, bucket_name: String, key: String) -> Self {
        create_error_response(
            ErrorCode::NoSuchKey,
            format!("/{}/{}", bucket_name, key),
            request_id,
            Some(key),
            Some(bucket_name),
        )
    }

    pub fn internal_error(request_id: String) -> Self {
        create_error_response(ErrorCode::InternalError, "/".to_string(), request_id, None, None)
    }
}