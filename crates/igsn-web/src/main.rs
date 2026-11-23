use axum::Router;
use igsn_web::App;
use leptos::prelude::*;
use leptos_axum::{LeptosRoutes, file_and_error_handler, generate_route_list};

fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <title>"GROVER"</title>
                <link rel="icon" href="data:,"/>
                <link rel="stylesheet" id="leptos" href="/pkg/grover.css"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
            </head>
            <body><App/></body>
        </html>
    }
}

fn router(options: LeptosOptions) -> Router {
    let routes = generate_route_list(App);
    let shell_options = options.clone();
    Router::new()
        .leptos_routes(&options, routes, move || shell(shell_options.clone()))
        .fallback(file_and_error_handler(shell))
        .with_state(options)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = get_configuration(None)?.leptos_options;
    let listener = tokio::net::TcpListener::bind(options.site_addr).await?;
    println!("Serving GROVER on http://{}", listener.local_addr()?);
    axum::serve(listener, router(options)).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::{Body, to_bytes};
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn home_serves_the_development_notice_in_the_initial_html() {
        let options = LeptosOptions::builder().output_name("grover").build();
        let request = Request::get("/").body(Body::empty()).unwrap();
        let response = router(options).oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body(), 64 * 1024).await.unwrap();
        let html = std::str::from_utf8(&body).unwrap();
        assert!(html.contains("<title>GROVER</title>"));
        assert!(html.contains("<p>This is where GROVER is being developed - the Generator of Researcher-friendly, Ontology-aligned, Validated Excel templates for Research samples!</p>"));
    }
}
