mod common;
use axum::http::StatusCode;
use common::TestApp;

#[tokio::test]
async fn create_returns_201_and_json() {
    let app = TestApp::new();

    let (status, json) = app.create_short_url("https://example.com", 60).await;

    assert_eq!(status, StatusCode::CREATED);
    assert!(json.get("id").is_some());
    assert_eq!(json.get("original").unwrap(), "https://example.com");
}
