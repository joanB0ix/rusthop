#![cfg(feature = "memory")]

use std::sync::Arc;

use async_trait::async_trait;
use dashmap::DashMap;

use crate::{
    domain::ShortUrl,
    ports::{RepoError, UrlRepository},
};

#[derive(Clone, Default)]
pub struct InMemoryRepo {
    map: Arc<DashMap<String, ShortUrl>>,
}

#[async_trait]
impl UrlRepository for InMemoryRepo {
    async fn save(&self, url: ShortUrl) -> Result<(), RepoError> {
        self.map.insert(url.id.clone(), url);
        Ok(())
    }

    async fn find(&self, id: &str) -> Result<ShortUrl, RepoError> {
        self.map
            .get(id)
            .map(|e| e.value().clone())
            .ok_or(RepoError::NotFound)
    }

    async fn increment_hit(&self, id: &str) -> Result<(), RepoError> {
        self.map.get_mut(id).map(|mut e| e.hits += 1);
        Ok(())
    }

    async fn delete(&self, id: &str) -> Result<(), RepoError> {
        self.map.remove(id);
        Ok(())
    }
}
