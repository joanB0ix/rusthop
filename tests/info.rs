mod common;
use axum::http::StatusCode;
use common::TestApp;

#[tokio::test]
async fn info_endpoint_returns_hits_without_incrementing() {
    let app = TestApp::new();

    let (_, created) = app.create_short_url("https://example.com", 60).await;
    let id = created.get("id").unwrap().as_str().unwrap();

    let (status, info) = app.get_info(id).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(info.get("hits").unwrap(), 0);

    let _ = app.resolve(id).await;

    let (_, info) = app.get_info(id).await;
    assert_eq!(info.get("hits").unwrap(), 1);
}
