use super::*;
use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode, header};
use tower::ServiceExt;

fn app() -> Router {
    router(LeptosOptions::builder().output_name("grover").build())
}

async fn initial_html(path: &str, expected_status: StatusCode) -> String {
    let request = Request::get(path).body(Body::empty()).unwrap();
    let response = app().oneshot(request).await.unwrap();
    assert_eq!(response.status(), expected_status);
    assert_eq!(
        response.headers()[header::CONTENT_TYPE],
        "text/html; charset=utf-8"
    );

    let body = to_bytes(response.into_body(), 64 * 1024).await.unwrap();
    String::from_utf8(body.to_vec()).unwrap()
}

#[tokio::test]
async fn home_serves_the_development_notice_in_the_initial_html() {
    let html = initial_html("/", StatusCode::OK).await;
    assert!(html.contains("<title>GROVER</title>"));
    assert!(html.contains("<p>This is where GROVER is being developed - the Generator of Researcher-friendly, Ontology-aligned, Validated Excel templates for Research samples!</p>"));
}

#[tokio::test]
async fn changelog_serves_the_current_release_in_the_initial_html() {
    let html = initial_html("/changelog", StatusCode::OK).await;
    let (main, _) = html.split_once("</main>").unwrap();
    let (navigation, _) = main
        .split_once("<h1>Release notes for GROVER</h1>")
        .unwrap();
    assert!(navigation.contains(r#"href="/""#));
    assert!(navigation.contains("Back to GROVER"));
    assert!(main.contains("v0.1.0dev"));
    assert!(main.contains("Unreleased"));
    assert!(main.contains("Added"));
    assert!(main.contains("Open the release notes from the version link in the footer"));
}

#[tokio::test]
async fn the_home_changelog_and_not_found_pages_share_one_footer_after_main() {
    fn footer(html: &str) -> &str {
        assert_eq!(html.matches("<footer").count(), 1);
        let (_, after_main) = html.split_once("</main>").unwrap();
        let (_, content) = after_main.split_once("<footer").unwrap();
        content.split_once("</footer>").unwrap().0
    }
    let home = initial_html("/", StatusCode::OK).await;
    let changelog = initial_html("/changelog", StatusCode::OK).await;
    let missing = initial_html("/missing-page", StatusCode::NOT_FOUND).await;
    assert!(missing.contains(r#"<div class="page-notice"><p>Page not found."#));
    let home_footer = footer(&home);
    assert_eq!(home_footer, footer(&changelog));
    assert_eq!(home_footer, footer(&missing));
    assert!(home_footer.contains(r#"href="/changelog""#));
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
