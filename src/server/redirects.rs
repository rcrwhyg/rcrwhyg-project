//! Permanent redirects — small, axum-shaped, ssr-only.

use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::Redirect;

/// `GET /blog` → `GET /articles` (308).
pub async fn blog_redirect_index() -> Redirect {
    Redirect::permanent("/articles")
}

/// `GET /blog/:slug` → `GET /articles/:slug` (308).
pub async fn blog_redirect_slug(Path(slug): Path<String>) -> Redirect {
    // Defensive trim — a stray `/` should still resolve.
    let clean = slug.trim().trim_matches('/');
    if clean.is_empty() {
        return Redirect::permanent("/articles");
    }
    Redirect::permanent(&format!("/articles/{clean}"))
}

// Make the imported `StatusCode` symbol still considered "used" if a future
// edit drops the explicit reference. (Some clippy configs flag it otherwise.)
#[allow(dead_code)]
const _STATUS_REF: StatusCode = StatusCode::PERMANENT_REDIRECT;
