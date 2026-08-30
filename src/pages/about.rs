use leptos::prelude::*;

use crate::server::{AboutContent, get_about};

/// `/about` — markdown body rendered server-side from `content/about.md`.
#[component]
pub fn AboutPage() -> impl IntoView {
    let content = Resource::new(|| (), |_| async move { get_about().await });

    view! {
        // 跟其他公开页一致：section 套 page-panel 拿到 0.30 玻璃面，
        // 文字落在稳定的"纸面"上而不是直接压到背景渐变上。
        <section class="page-panel mx-auto my-8 max-w-4xl px-4">
            // 不再展示 "关于" h1——直接呈现 markdown 内容。
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
