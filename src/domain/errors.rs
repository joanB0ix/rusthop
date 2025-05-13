use super::MAX_TTL_SECS;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UrlError {
    #[error("invalid URL")]
    InvalidUrl,
    #[error("TTL exceeds {MAX_TTL_SECS} seconds")]
    TtlTooLong,
}
