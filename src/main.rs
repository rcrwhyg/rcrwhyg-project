#[cfg(feature = "ssr")]
use axum::body::Body;
#[cfg(feature = "ssr")]
use axum::extract::Request;
#[cfg(feature = "ssr")]
use axum::http::StatusCode;
#[cfg(feature = "ssr")]
use axum::middleware::Next;
#[cfg(feature = "ssr")]
use axum::response::Response;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::net::SocketAddr;

    use axum::Router;
    use axum::middleware;
    use axum::routing::get;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{LeptosRoutes, generate_route_list};
    use rcrwhyg_server::app::*;
    use rcrwhyg_server::server::{
        AppState, blog_redirect_index, blog_redirect_slug, global_rate_limit_middleware, health,
        sse_heartbeat, ws_echo,
    };
    use tokio::net::TcpListener;

    dotenvy::dotenv().ok();

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    let app_state = AppState::new(leptos_options).await;

    let app = Router::new()
        .leptos_routes_with_context(
            &app_state,
            routes,
            {
                let app_state = app_state.clone();
                move || app_state.provide_leptos_context()
            },
            {
                let leptos_options = app_state.leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .route("/health", get(health))
        .route("/sse/heartbeat", get(sse_heartbeat))
        .route("/ws/echo", get(ws_echo))
        // `/blog` → `/articles` (308 — preserves method, SEO-friendly).
        .route("/blog", get(blog_redirect_index))
        .route("/blog/{slug}", get(blog_redirect_slug))
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .layer(middleware::from_fn(strip_trailing_slash))
        .layer(middleware::from_fn(global_rate_limit_middleware))
        .with_state(app_state);

    log!("Server listening on {}", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}

/// 统一去掉非根路径的尾部斜杠：`/articles/` → `/articles`（308）。
/// 尾部斜杠会命出 thaw 的 SSRMountStyleProvider panic 分支（"without a <head>"），
/// 归一化后保证 `/articles/xx/` 与 `/articles/xx` 解析一致。
#[cfg(feature = "ssr")]
async fn strip_trailing_slash(req: Request, next: Next) -> Response {
    let path = req.uri().path();
    if path.len() > 1 && path.ends_with('/') {
        let mut location = path.trim_end_matches('/').to_string();
        if let Some(query) = req.uri().query() {
            location.push('?');
            location.push_str(query);
        }
        return Response::builder()
            .status(StatusCode::TEMPORARY_REDIRECT)
            .header("location", location)
            .body(Body::empty())
            .expect("empty body response");
    }
    next.run(req).await
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // Client entry lives in lib.rs hydrate().
}
