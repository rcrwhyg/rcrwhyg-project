use leptos::prelude::*;
use leptos_router::components::A;

use crate::server::{RadarEntry, list_radar};

/// `/radar` — multi-ecosystem learning progress (mint = action, sky = info).
#[component]
pub fn RadarPage() -> impl IntoView {
    let entries = Resource::new(|| (), |_| async move { list_radar().await });

    view! {
        <section class="page-panel mx-auto my-8 max-w-3xl px-4">
            <h1 class="page-title mb-2">
                <span class="accent-2">"学习雷达"</span>
            </h1>
            <p class="mb-6 muted-text">
                "多生态学习进度。色条=主色：薄荷=行动/深入，天空=信息/了解。"
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
                        <p class="dim-text">"暂无雷达数据。"</p>
                    }
                    .into_any(),
                    Some(Ok(items)) => view! {
                        <div class="space-y-3">
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
    let accent_class = if entry.accent.eq_ignore_ascii_case("sky") {
        "s-sky"
    } else {
        "s-mint"
    };
    let has_link = entry.link.is_some();
    let link = entry.link.clone().unwrap_or_default();
    let note = entry.note.clone().unwrap_or_default();
    let has_note = !note.is_empty();

    view! {
        <div class=format!("radar-row {accent_class}")>
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
