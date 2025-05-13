mod common;
use axum::http::StatusCode;
use common::TestApp;
use url::Url;

#[tokio::test]
async fn resolve_redirects_to_original() {
    let app = TestApp::new();

    let (_, created) = app.create_short_url("https://example.com", 60).await;
    let id = created.get("id").unwrap().as_str().unwrap();

    let (status, location) = app.resolve(id).await;

    assert_eq!(status, StatusCode::TEMPORARY_REDIRECT);
    let dest = Url::parse(location.as_ref().unwrap()).unwrap();
    assert_eq!(dest.as_str(), "https://example.com/");
}
