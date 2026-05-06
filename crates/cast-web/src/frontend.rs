//! Embedded frontend bundle. `tsc` compiles `web/src/*.ts` to
//! `web/dist/*.js` 1:1 and `index.html` lives at `web/index.html`;
//! `rust-embed` bakes the result into the release binary so a
//! single artefact ships the full UI.
//!
//! Pass 1: the `web/` tree is hand-written stubs. The `tsc` step
//! is wired in package.json but the embedded bundle is whatever is
//! present at compile time — an empty `dist/` is acceptable for the
//! first scaffold; the in-tree `index.html` already drives a usable
//! page via the `dist/` it imports.

use axum::body::Body;
use axum::extract::Path as AxumPath;
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "web/"]
#[include = "index.html"]
#[include = "dist/*"]
struct Frontend;

pub async fn index() -> impl IntoResponse {
    serve("index.html").into_response()
}

pub async fn asset(AxumPath(path): AxumPath<String>) -> impl IntoResponse {
    serve(&format!("dist/{path}")).into_response()
}

fn serve(path: &str) -> Response {
    match Frontend::get(path) {
        Some(file) => {
            let mime = mime_for(path);
            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, mime)
                .body(Body::from(file.data.into_owned()))
                .unwrap()
        }
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

fn mime_for(path: &str) -> &'static str {
    match path.rsplit('.').next() {
        Some("html") => "text/html; charset=utf-8",
        Some("js") | Some("mjs") => "application/javascript; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("json") => "application/json",
        Some("svg") => "image/svg+xml",
        _ => "application/octet-stream",
    }
}
