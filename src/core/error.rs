use reqwest;
use serde_json;
use thiserror::Error;
use url;
use url::ParseError;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Chartmuseum endpoint config error, [{0}]")]
    EndpointConfigError(String),
    #[error("Error when make request to Chartmuseum, message: [{0}]")]
    ClientError(String),
    #[error("Error parsing response json in line: {line:?} column: {column:?}")]
    JsonParseError {
        line: usize,
        column: usize,
        category: String,
    },
    #[error("Not found")]
    NotFound,
}

impl From<reqwest::Error> for CoreError {
    fn from(e: reqwest::Error) -> Self {
        if e.is_status() && e.status().unwrap() == 404 {
            return CoreError::NotFound;
        }
        CoreError::ClientError(e.to_string())
    }
}

impl From<serde_json::Error> for CoreError {
    fn from(e: serde_json::Error) -> Self {
        CoreError::JsonParseError {
            line: e.line(),
            column: e.column(),
            category: format!("{:?}", e.classify()),
        }
    }
}

impl From<url::ParseError> for CoreError {
    fn from(e: ParseError) -> Self {
        CoreError::EndpointConfigError(format!("{:?}", e))
    }
}
