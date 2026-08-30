use leptos::prelude::*;
use leptos_router::components::A;

use crate::server::{RadarEntry, list_radar};

/// `/radar` — multi-ecosystem learning progress (mint = action, sky = info).
#[component]
pub fn RadarPage() -> impl IntoView {
    let entries = Resource::new(|| (), |_| async move { list_radar().await });

    view! {
        // /radar 列表页：不再用 page-panel 框，宽度 max-w-4xl 与首页一致；
        // 每行是独立的 radar-row（带 accent 色条 + frosted glass）。
        <section class="mx-auto my-8 max-w-4xl space-y-3 px-4">
            // 列表直接呈现，不再展示 "学习雷达" h1 / 描述——eyebrow 已经够。
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
                        <p class="dim-text">"暂无雷达数据。"</p>
                    }
                    .into_any(),
                    Some(Ok(items)) => view! {
                        <div class="space-y-5">
                            {items
                                .into_iter()
                                .map(|entry| view! { <RadarRow entry=entry /> })
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
fn RadarRow(entry: RadarEntry) -> impl IntoView {
    let row_class = if entry.accent.eq_ignore_ascii_case("sky") {
        "radar-row s-sky"
    } else {
        "radar-row s-mint"
    };
    let has_link = entry.link.is_some();
    let link = entry.link.clone().unwrap_or_default();
    let note = entry.note.clone().unwrap_or_default();
    let has_note = !note.is_empty();

    view! {
        <div class=row_class>
            <div>
                <div class="name">{entry.ecosystem}</div>
                <Show when=move || has_note fallback=|| ()>
                    <div class="note mt-1">{note.clone()}</div>
                </Show>
            </div>
            <div>
                <Show when=move || has_link fallback=|| ()>
                    <A
                        href=link.clone()
                        attr:target="_blank"
                        attr:rel="noreferrer"
                        attr:class="link-text text-sm"
                    >
                        "→ 资料"
                    </A>
                </Show>
            </div>
            <div class="status">{entry.status}</div>
        </div>
    }
}
