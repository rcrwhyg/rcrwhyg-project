use leptos::prelude::*;
use leptos_router::components::A;

use crate::server::{LabEntry, list_lab};

/// `/lab` — experiment / small demos (mint accent = action / interaction).
#[component]
pub fn LabPage() -> impl IntoView {
    let entries = Resource::new(|| (), |_| async move { list_lab().await });

    view! {
        <section class="page-panel mx-auto my-8 max-w-3xl px-4">
            <h1 class="page-title mb-2">
                <span class="accent">"实验室"</span>
            </h1>
            <p class="mb-6 muted-text">
                "实验 / demo / 小游戏。点开卡片进入对应实验。"
            </p>

            <Suspense fallback=move || {
                view! { <p class="dim-text">"加载中…"</p> }
            }>
                {move || match entries.get() {
                    None => view! { <p class="dim-text">"加载中…"</p> }.into_any(),
                    Some(Err(err)) => view! {
                        <p class="danger-text">{format!("加载失败：{err}")}</p>
                    }
                    .into_any(),
                    Some(Ok(items)) if items.is_empty() => view! {
                        <p class="dim-text">"暂无实验项目。"</p>
                    }
                    .into_any(),
                    Some(Ok(items)) => view! {
                        <div class="section-grid">
                            {items
                                .into_iter()
                                .map(|entry| view! { <LabCard entry=entry /> })
                                .collect_view()}
                        </div>
                    }
                    .into_any(),
                }}
            </Suspense>
        </section>
    }
}

#[component]
fn LabCard(entry: LabEntry) -> impl IntoView {
    let has_link = entry.link.is_some();
    let link = entry.link.clone().unwrap_or_default();
    let status_text = entry.status.clone();

    view! {
        <article class="lab-card">
            <span class="stack">{entry.stack}</span>
            <h3>{entry.name}</h3>
            <p>{entry.blurb}</p>
            <div class="mt-1 flex items-center justify-between">
                <span class="tag">{status_text}</span>
                <Show when=move || has_link fallback=|| ()>
                    <A
                        href=link.clone()
                        attr:target="_blank"
                        attr:rel="noreferrer"
                        attr:class="link-text text-sm"
                    >
                        "查看 →"
                    </A>
                </Show>
            </div>
        </article>
    }
}
