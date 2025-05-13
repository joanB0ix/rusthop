use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use rusthop::{
    adapters::inbound::http::HttpServer, adapters::outbound::in_memory::InMemoryRepo,
    application::ShortenerService, shared::NanoIdGenerator,
};
use serde_json::Value;
use std::sync::Arc;
use tower::ServiceExt as _;

pub struct TestApp {
    router: Router,
}

impl TestApp {
    pub fn new() -> Self {
        let repo = InMemoryRepo::default();
        let svc = ShortenerService::new(repo, Arc::new(NanoIdGenerator));
        let router = HttpServer::new(svc).router();
        Self { router }
    }

    pub async fn create_short_url(&self, url: &str, ttl_secs: u64) -> (StatusCode, Value) {
        let payload = serde_json::json!({ "url": url, "ttl_secs": ttl_secs });

        let response = self
            .router
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/")
                    .header("content-type", "application/json")
                    .body(Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        let status = response.status();
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();

        (status, json)
    }

    pub async fn resolve(&self, id: &str) -> (StatusCode, Option<String>) {
        let response = self
            .router
            .clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(&format!("/{id}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let status = response.status();
        let location = response
            .headers()
            .get("location")
            .map(|v| v.to_str().unwrap().to_string());

        (status, location)
    }

    pub async fn get_info(&self, id: &str) -> (StatusCode, Value) {
        let response = self
            .router
            .clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(&format!("/api/urls/{id}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let status = response.status();
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();

        (status, json)
    }
}
