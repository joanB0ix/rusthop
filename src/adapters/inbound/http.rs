use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
};
use serde::Deserialize;

use crate::{application::ShortenerService, ports::UrlRepository};

#[derive(Clone)]
pub struct AppState<S: UrlRepository> {
    service: Arc<ShortenerService<S>>,
}

pub struct HttpServer<S: UrlRepository + Clone + Send + Sync + 'static> {
    state: AppState<S>,
}

impl<S> HttpServer<S>
where
    S: UrlRepository + Clone + Send + Sync + 'static,
{
    pub fn new(service: ShortenerService<S>) -> Self {
        Self {
            state: AppState {
                service: Arc::new(service),
            },
        }
    }

    pub fn router(self) -> Router {
        Router::new()
            .route("/", post(Self::shorten))
            .route("/{id}", get(Self::resolve))
            .route("/api/urls/{id}", get(Self::get_info))
            .with_state(self.state)
    }

    async fn shorten(
        State(state): State<AppState<S>>,
        Json(body): Json<ShortenRequest>,
    ) -> impl IntoResponse {
        match state.service.shorten(body.url, body.ttl_secs).await {
            Ok(short) => (StatusCode::CREATED, Json(short)).into_response(),
            Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
        }
    }

    async fn resolve(
        State(state): State<AppState<S>>,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        match state.service.resolve(&id).await {
            Ok(dest) => Redirect::temporary(&dest).into_response(),
            Err(_) => StatusCode::NOT_FOUND.into_response(),
        }
    }

    async fn get_info(
        State(state): State<AppState<S>>,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        match state.service.info(&id).await {
            Ok(short) => (StatusCode::OK, Json(short)).into_response(),
            Err(_) => StatusCode::NOT_FOUND.into_response(),
        }
    }
}

#[derive(Deserialize)]
struct ShortenRequest {
    url: String,
    ttl_secs: u64,
}
