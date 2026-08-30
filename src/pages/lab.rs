use leptos::prelude::*;
use leptos_router::components::A;

use crate::server::{LabEntry, list_lab};

/// `/lab` — 实验室。完全复用首页轮播的 section-card + s-mint
/// 视觉（顶部 mint 色条 + 玻璃底框 + backdrop-filter），数据形状
/// 由 LabEntry 提供 (stack / name / blurb / status / link)。
#[component]
pub fn LabPage() -> impl IntoView {
    let entries = Resource::new(|| (), |_| async move { list_lab().await });

    view! {
        // /lab 列表页：去掉 page-panel 框，宽度 max-w-4xl 与首页一致。
        <section class="mx-auto my-8 max-w-4xl px-4">
            // 不再展示 h1 / 描述。
            <Suspense fallback=move || {
                view! { <p class="dim-text">"加载中…"</p> }
            }>
                {move || match entries.get() {
                    None => view! { <p class="dim-text">"加载中…"</p> }.into_any(),
                    Some(Err(err)) => view! {
                        <p class="danger-text">{format!("加载失败：{err}")}</p> }
                    .into_any(),
                    Some(Ok(items)) if items.is_empty() => view! {
                        <p class="dim-text">"暂无实验项目。"</p>
                    }
                    .into_any(),
                    Some(Ok(items)) => view! {
                        <div class="section-carousel">
                            <div class="section-carousel__track">
                                {items
                                    .iter()
                                    .map(|entry| view! { <LabCard entry=entry.clone() /> })
                                    .collect_view()}
                                {items
                                    .iter()
                                    .map(|entry| view! { <LabCard entry=entry.clone() /> })
                                    .collect_view()}
                            </div>
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
        <article class="section-card s-mint">
            <span class="eyebrow">{entry.stack}</span>
            <h3>{entry.name}</h3>
            <p>{entry.blurb}</p>
            <div class="mt-2 flex items-center justify-between">
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
