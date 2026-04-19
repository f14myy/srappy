use serde::Serialize;

#[derive(Debug, thiserror::Error, Serialize)]
pub enum ScrapeError {
    #[error("Network error: {0}")]
    Network(String),
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Blocked or Timeout: {0}")]
    BlockedOrTimeout(String),
    #[error("File write error: {0}")]
    File(String),
    #[error("Unexpected error: {0}")]
    Other(String),
}
