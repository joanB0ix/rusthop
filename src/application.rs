use std::sync::Arc;

use crate::{
    domain::ShortUrl,
    id::IdGenerator,
    ports::{RepoError, UrlRepository},
};

pub struct ShortenerService<R: UrlRepository> {
    repo: R,
    generator: Arc<dyn IdGenerator>,
}

impl<R: UrlRepository> ShortenerService<R> {
    pub fn new(repo: R, generator: Arc<dyn IdGenerator>) -> Self {
        Self { repo, generator }
    }

    pub async fn shorten(&self, original: String, ttl_secs: u64) -> Result<ShortUrl, RepoError> {
        let id = self.generator.generate();
        let short_url =
            ShortUrl::new(id, original, ttl_secs).map_err(|e| RepoError::Other(e.into()))?;

        self.repo.save(short_url.clone()).await?;
        Ok(short_url)
    }

    pub async fn resolve(&self, id: &str) -> Result<String, RepoError> {
        let url = self.repo.find(id).await?;

        if url.is_expired() {
            self.repo.delete(id).await.ok();
            return Err(RepoError::NotFound);
        }

        self.repo.increment_hit(id).await?;
        Ok(url.original)
    }
}
