use mcp_core_fishcode2025::handler::ResourceError;
use mcp_core_fishcode2025::protocol::{
    ErrorData, INTERNAL_ERROR, INVALID_PARAMS, INVALID_REQUEST, METHOD_NOT_FOUND,
};
use thiserror::Error;

pub type BoxError = Box<dyn std::error::Error + Sync + Send>;

#[derive(Error, Debug)]
pub enum TransportError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Invalid UTF-8 sequence: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("Protocol error: {0}")]
    Protocol(String),

    #[error("Invalid message format: {0}")]
    InvalidMessage(String),
}

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Transport error: {0}")]
    Transport(#[from] TransportError),

    #[error("Service error: {0}")]
    Service(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Request timed out")]
    Timeout(#[from] tower::timeout::error::Elapsed),
}

#[derive(Error, Debug)]
pub enum RouterError {
    #[error("Method not found: {0}")]
    MethodNotFound(String),

    #[error("Invalid parameters: {0}")]
    InvalidParams(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Tool not found: {0}")]
    ToolNotFound(String),

    #[error("Resource not found: {0}")]
    ResourceNotFound(String),

    #[error("Not found: {0}")]
    PromptNotFound(String),
}

impl From<RouterError> for ErrorData {
    fn from(err: RouterError) -> Self {
        match err {
            RouterError::MethodNotFound(msg) => ErrorData {
                code: METHOD_NOT_FOUND,
                message: msg,
                data: None,
            },
            RouterError::InvalidParams(msg) => ErrorData {
                code: INVALID_PARAMS,
                message: msg,
                data: None,
            },
            RouterError::Internal(msg) => ErrorData {
                code: INTERNAL_ERROR,
                message: msg,
                data: None,
            },
            RouterError::ToolNotFound(msg) => ErrorData {
                code: INVALID_REQUEST,
                message: msg,
                data: None,
            },
            RouterError::ResourceNotFound(msg) => ErrorData {
                code: INVALID_REQUEST,
                message: msg,
                data: None,
            },
            RouterError::PromptNotFound(msg) => ErrorData {
                code: INVALID_REQUEST,
                message: msg,
                data: None,
            },
        }
    }
}

impl From<ResourceError> for RouterError {
    fn from(err: ResourceError) -> Self {
        match err {
            ResourceError::NotFound(msg) => RouterError::ResourceNotFound(msg),
            ResourceError::ExecutionError(msg) => {
                RouterError::Internal(format!("Resource execution error: {}", msg))
            }
        }
    }
}
