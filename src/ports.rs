use async_trait::async_trait;
use thiserror::Error;

use crate::domain::ShortUrl;

#[derive(Debug, Error)]
pub enum RepoError {
    #[error("Short-link not found")]
    NotFound,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type RepoResult<T> = Result<T, RepoError>;

#[async_trait]
pub trait UrlRepository: Send + Sync {
    async fn save(&self, url: ShortUrl) -> RepoResult<()>;
    async fn find(&self, id: &str) -> RepoResult<ShortUrl>;
    async fn increment_hit(&self, id: &str) -> RepoResult<()>;
    async fn delete(&self, id: &str) -> RepoResult<()>;
}
