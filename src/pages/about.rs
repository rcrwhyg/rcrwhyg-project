use leptos::prelude::*;

use crate::server::{AboutContent, get_about};

/// `/about` — markdown body rendered server-side from `content/about.md`.
#[component]
pub fn AboutPage() -> impl IntoView {
    let content = Resource::new(|| (), |_| async move { get_about().await });

    view! {
        <section class="page-panel mx-auto my-8 max-w-3xl px-4">
            <h1 class="page-title mb-6">
                <span class="accent">"关于"</span>
            </h1>
            <Suspense fallback=move || {
                view! { <p class="dim-text">"加载中…"</p> }
            }>
                {move || match content.get() {
                    None => view! { <p class="dim-text">"加载中…"</p> }.into_any(),
                    Some(Err(err)) => view! {
                        <p class="danger-text">{format!("加载失败：{err}")}</p>
                    }
                    .into_any(),
                    Some(Ok(None)) => view! {
                        <p class="dim-text">"暂无内容（content/about.md 缺失）。"</p>
                    }
                    .into_any(),
                    Some(Ok(Some(AboutContent { body_html }))) => view! {
                        <div class="prose-site" inner_html=body_html></div>
                    }
                    .into_any(),
                }}
            </Suspense>
        </section>
    }
}
