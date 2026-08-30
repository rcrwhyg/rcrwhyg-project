use leptos::prelude::*;

use crate::server::{MusicEntry, list_music};

/// `/music` — playlist. Each row is self-hosted `<audio>` when `src` is set,
/// otherwise an iframe embed (Spotify / Apple Music / etc.), otherwise a
/// placeholder note.
#[component]
pub fn MusicPage() -> impl IntoView {
    let entries = Resource::new(|| (), |_| async move { list_music().await });

    view! {
        // /music 列表页：去掉 page-panel 框，宽度 max-w-4xl 与首页一致。
        <section class="mx-auto my-8 max-w-4xl space-y-3 px-4">
            // 不再展示 "音乐" h1 / 描述——直接呈现列表，card 内部已经带曲目信息。
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
                        <p class="dim-text">"暂无曲目。"</p>
                    }
                    .into_any(),
                    Some(Ok(items)) => view! {
                        <div class="space-y-3">
                            {items
                                .into_iter()
                                .map(|entry| view! { <MusicRow entry=entry /> })
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
fn MusicRow(entry: MusicEntry) -> impl IntoView {
    let note = entry.note.clone().unwrap_or_default();
    let has_note = !note.is_empty();
    let title = entry.title.clone();
    let artist = entry.artist.clone();

    // Render variants based on what's set. The HTML is server-side so we
    // can read the optionals without a runtime check on the client.
    let body = match (entry.src.as_deref(), entry.embed_url.as_deref()) {
        (Some(src), _) => view! {
            <audio controls preload="none" class="mt-3 w-full">
                <source src=src.to_string() />
            </audio>
        }
        .into_any(),
        (None, Some(embed)) => view! {
            <div class="mt-3 aspect-video w-full overflow-hidden rounded border border-[color:var(--glass-border)]">
                <iframe
                    src=embed.to_string()
                    title=format!("{artist} · {title}")
                    class="h-full w-full"
                ></iframe>
            </div>
        }
        .into_any(),
        (None, None) => view! {
            <p class="mt-3 dim-text">
                {note.clone()}
            </p>
        }
        .into_any(),
    };

    view! {
        <article class="music-row">
            <header class="flex items-baseline justify-between gap-3">
                <h3>{title}</h3>
                <span class="artist">{artist}</span>
            </header>
            {body}
            <Show when=move || has_note>
                <p class="mt-2 text-sm dim-text">{note.clone()}</p>
            </Show>
        </article>
    }
}
