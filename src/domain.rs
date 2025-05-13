use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const MAX_TTL_SECS: u64 = 3600;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortUrl {
    pub id: String,
    pub original: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub hits: u64,
}

impl ShortUrl {
    pub fn new(id: String, original: String, ttl_secs: u64) -> Result<Self, UrlError> {
        url::Url::parse(&original).map_err(|_| UrlError::InvalidUrl)?;

        if ttl_secs == 0 || ttl_secs > MAX_TTL_SECS {
            return Err(UrlError::TtlTooLong);
        }

        let created_at = Utc::now();
        let expires_at = created_at + Duration::seconds(ttl_secs as i64);

        Ok(Self {
            id,
            original,
            created_at,
            expires_at,
            hits: 0,
        })
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
}

#[derive(Debug, Error)]
pub enum UrlError {
    #[error("Invalid Url")]
    InvalidUrl,
    #[error("TTL exceeds {MAX_TTL_SECS} seconds")]
    TtlTooLong,
}
