use super::*;
use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode, header};
use tower::ServiceExt;

fn app() -> Router {
    router(LeptosOptions::builder().output_name("grover").build())
}

#[tokio::test]
async fn home_serves_the_development_notice_in_the_initial_html() {
    let request = Request::get("/").body(Body::empty()).unwrap();
    let response = app().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers()[header::CONTENT_TYPE],
        "text/html; charset=utf-8"
    );

    let body = to_bytes(response.into_body(), 64 * 1024).await.unwrap();
    let html = std::str::from_utf8(&body).unwrap();
    assert!(html.contains("<title>GROVER</title>"));
    assert!(html.contains("<p>This is where GROVER is being developed - the Generator of Researcher-friendly, Ontology-aligned, Validated Excel templates for Research samples!</p>"));
}

#[tokio::test]
async fn a_missing_browser_asset_returns_not_found() {
    let request = Request::get("/pkg/missing-asset.wasm")
        .body(Body::empty())
        .unwrap();
    let response = app().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn the_home_page_rejects_post_requests() {
    let request = Request::post("/").body(Body::empty()).unwrap();
    let response = app().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
}
