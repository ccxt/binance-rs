use serde::Deserialize;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, SdkError>;

#[derive(Error, Debug)]
pub enum SdkError {
    #[error("{0}")]
    BinanceError(#[from] BinanceContentError),

    #[error("{name} at {index} is missing")]
    KlineValueMissingError { name: String, index: usize },

    // Box the large error types
    #[error("Request error")]
    ReqError(#[from] Box<reqwest::Error>),

    #[error("Tungstenite error")]
    Tungstenite(#[from] Box<tungstenite::Error>),

    // Keep small errors unboxed
    #[error("Invalid header error")]
    InvalidHeaderError(#[from] reqwest::header::InvalidHeaderValue),

    #[error("IO error")]
    IoError(#[from] std::io::Error),

    #[error("Parse float error")]
    ParseFloatError(#[from] std::num::ParseFloatError),

    #[error("URL parser error")]
    UrlParserError(#[from] url::ParseError),

    #[error("JSON error")]
    Json(#[from] serde_json::Error),

    #[error("Timestamp error")]
    TimestampError(#[from] std::time::SystemTimeError),

    #[error("{0}")]
    Other(String),
}

impl From<reqwest::Error> for SdkError {
    fn from(err: reqwest::Error) -> Self {
        SdkError::ReqError(Box::new(err))
    }
}

impl From<tungstenite::Error> for SdkError {
    fn from(err: tungstenite::Error) -> Self {
        SdkError::Tungstenite(Box::new(err))
    }
}

impl From<String> for SdkError {
    fn from(err: String) -> Self {
        SdkError::Other(err)
    }
}

#[derive(Error, Debug, Clone, Deserialize)]
#[error("Binance content error: {msg} (code: {code})")]
pub struct BinanceContentError {
    pub code: i16,
    pub msg: String,
}
