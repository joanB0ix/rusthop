use async_trait::async_trait;
use thiserror::Error;

use crate::domain::ShortUrl;

#[derive(Debug, Error)]
pub enum RepoError {
    #[error("short‑link not found")]
    NotFound,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[async_trait]
pub trait UrlRepository: Send + Sync {
    async fn save(&self, url: ShortUrl) -> Result<(), RepoError>;
    async fn find(&self, id: &str) -> Result<ShortUrl, RepoError>;
    async fn increment_hit(&self, id: &str) -> Result<(), RepoError>;
    async fn delete(&self, id: &str) -> Result<(), RepoError>;
}
