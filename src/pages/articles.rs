use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;

use crate::server::{ArticleMeta, CollectionPlaceholder, get_site_article, list_site_articles};

/// `/articles` — 倒序 flat 列表；合集由子目录 `_meta.json` 决定，徽章展示。
#[component]
pub fn ArticlesIndexPage() -> impl IntoView {
    let list = Resource::new(|| (), |_| async move { list_site_articles().await });

    view! {
        <section class="mx-auto my-8 max-w-4xl space-y-5 px-4">
            <Suspense fallback=move || {
                view! { <p class="text-[color:var(--fg-dim)]">"加载文章列表…"</p> }
            }>
                {move || match list.get() {
                    None => view! { <p class="text-[color:var(--fg-dim)]">"加载文章列表…"</p> }
                        .into_any(),
                    Some(Err(err)) => view! {
                        <p class="text-[color:var(--danger)]">{format!("加载失败：{err}")}</p>
                    }
                    .into_any(),
                    Some(Ok(view)) if view.articles.is_empty() && view.placeholders.is_empty() => {
                        view! { <p class="text-[color:var(--fg-dim)]">"暂无文章。"</p> }.into_any()
                    }
                    Some(Ok(view)) => view! {
                        <ul class="space-y-5">
                            {view
                                .placeholders
                                .into_iter()
                                .map(|p| view! { <CollectionPlaceholderRow item=p /> })
                                .collect_view()}
                            {view
                                .articles
                                .into_iter()
                                .map(|a| view! { <ArticleItem article=a /> })
                                .collect_view()}
                        </ul>
                    }
                    .into_any(),
                }}
            </Suspense>
        </section>
    }
}

#[component]
fn CollectionPlaceholderRow(item: CollectionPlaceholder) -> impl IntoView {
    let note = item.note.clone().unwrap_or_else(|| "敬请期待".to_string());
    let title = item.title.clone();
    view! {
        <li class="list-card opacity-80">
            <div class="flex flex-wrap items-baseline justify-between gap-3">
                <h2 class="text-base font-semibold text-[color:var(--fg-muted)]">{title}</h2>
                <span class="tag">"即将同步"</span>
            </div>
            <p class="mt-1 text-sm text-[color:var(--fg-dim)]">{note}</p>
        </li>
    }
}

#[component]
fn ArticleItem(article: ArticleMeta) -> impl IntoView {
    let href = format!("/articles/{}", article.slug);
    let date = article.date.clone().unwrap_or_default();
    let has_date = !date.is_empty();
    let has_summary = !article.summary.is_empty();
    let badge = article
        .collection_title
        .clone()
        .unwrap_or_else(|| "随笔".to_string());
    let badge_class = if article.collection_title.is_some() {
        "tag sky"
    } else {
        "tag"
    };

    view! {
        <li class="list-card">
            <div class="flex flex-wrap items-baseline justify-between gap-3">
                <h2 class="text-base font-semibold">
                    <A
                        href=href.clone()
                        attr:class="no-underline text-[color:var(--fg)] hover:text-[color:var(--fg-muted)]"
                    >
                        {article.title.clone()}
                    </A>
                </h2>
                <span class=badge_class>{badge.clone()}</span>
            </div>
            <Show when=move || has_date fallback=|| ()>
                <p class="mt-1 text-sm text-[color:var(--fg-dim)] font-mono">{date.clone()}</p>
            </Show>
            <Show when=move || has_summary fallback=|| ()>
                <p class="mt-1 text-sm text-[color:var(--fg-muted)]">{article.summary.clone()}</p>
            </Show>
        </li>
    }
}

#[component]
pub fn ArticlePage() -> impl IntoView {
    let params = use_params_map();
    let article = Resource::new(
        move || params.read().get("slug").unwrap_or_default(),
        |slug| async move { get_site_article(slug).await },
    );

    view! {
        <Suspense fallback=move || {
            view! {
                <section class="page-panel mx-auto my-8 max-w-3xl px-4">
                    <p class="text-[color:var(--fg-dim)]">"加载文章…"</p>
                </section>
            }
        }>
            {move || match article.get() {
                None => view! {
                    <section class="mx-auto my-8 max-w-4xl px-4">
                        <p class="dim-text">"加载文章…"</p>
                    </section>
                }
                .into_any(),
                Some(Err(err)) => view! {
                    <section class="mx-auto my-8 max-w-4xl px-4">
                        <p class="danger-text">{format!("加载失败：{err}")}</p>
                        <A href="/articles" attr:class="mt-4 inline-block text-sm link-text">
                            "返回文章"
                        </A>
                    </section>
                }
                .into_any(),
                Some(Ok(None)) => view! {
                    <section class="mx-auto my-8 max-w-4xl px-4">
                        <p class="link-text text-sm">
                            <A href="/articles">"← 返回文章"</A>
                        </p>
                    </section>
                }
                .into_any(),
                Some(Ok(Some(detail))) => {
                    let title = detail.meta.title.clone();
                    let meta = detail.meta.date.clone().unwrap_or_else(|| "未标注日期".into());
                    let badge = detail
                        .meta
                        .collection_title
                        .clone()
                        .unwrap_or_else(|| "随笔".to_string());
                    let badge_class = if detail.meta.collection_title.is_some() {
                        "tag sky"
                    } else {
                        "tag"
                    };
                    let body_html = detail.body_html.clone();
                    view! {
                        <article class="page-panel mx-auto my-8 max-w-4xl px-4">
                            <p class="mb-4">
                                <A href="/articles" attr:class="link-text text-sm">
                                    "← 文章"
                                </A>
                            </p>
                            <div class="mb-4 flex flex-wrap items-start justify-between gap-3">
                                <h1 class="page-title">{title}</h1>
                                <span class=badge_class>{badge.clone()}</span>
                            </div>
                            <p class="mb-8 text-sm dim-text font-mono">{format!("发布于 {meta}")}</p>
                            <div class="markdown-body" inner_html=body_html></div>
                        </article>
                    }
                    .into_any()
                }
            }}
        </Suspense>
    }
}
