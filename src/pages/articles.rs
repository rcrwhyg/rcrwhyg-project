use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;

use crate::server::{ArticleMeta, get_site_article, list_site_articles};

/// `/articles` — 个人网站首发文章索引（由 articles/*.md 生成）。
#[component]
pub fn ArticlesIndexPage() -> impl IntoView {
    let articles = Resource::new(|| (), |_| async move { list_site_articles().await });

    view! {
        // /articles 列表页：去掉 page-panel 框，宽度 max-w-4xl 与首页一致；
        // list-card 自带 frosted glass。
        <section class="mx-auto my-8 max-w-4xl space-y-5 px-4">
            // /articles 列表页：去掉 "文章" h1 + 描述，list 卡片直接呈现。
            // 列表语义已在 list-card 内部（每项的标题/日期/摘要/阅读全文），
            // 章节级标题是噪音。
            <Suspense fallback=move || {
                view! { <p class="text-[color:var(--fg-dim)]">"加载文章列表…"</p> }
            }>
                {move || match articles.get() {
                    None => view! { <p class="text-[color:var(--fg-dim)]">"加载文章列表…"</p> }
                        .into_any(),
                    Some(Err(err)) => view! {
                        <p class="text-[color:var(--danger)]">{format!("加载失败：{err}")}</p>
                    }
                    .into_any(),
                    Some(Ok(items)) if items.is_empty() => view! {
                        <p class="text-[color:var(--fg-dim)]">"暂无文章。"</p>
                    }
                    .into_any(),
                    Some(Ok(items)) => view! {
                        <ul class="space-y-5">
                            {items.into_iter().map(|a| view! { <ArticleItem article=a /> }).collect_view()}
                        </ul>
                    }
                    .into_any(),
                }}
            </Suspense>
        </section>
    }
}

#[component]
fn ArticleItem(article: ArticleMeta) -> impl IntoView {
    let href = format!("/articles/{}", article.slug);
    let date = article.date.clone().unwrap_or_default();
    let has_date = !date.is_empty();
    let has_summary = !article.summary.is_empty();

    view! {
        <li class="list-card">
            <h2 class="text-base font-semibold">
                <A
                    href=href.clone()
                    attr:class="no-underline text-[color:var(--fg)] hover:text-[color:var(--fg-muted)]"
                >
                    {article.title.clone()}
                </A>
            </h2>
            <Show when=move || has_date fallback=|| ()>
                <p class="mt-1 text-sm text-[color:var(--fg-dim)] font-mono">{date.clone()}</p>
            </Show>
            <Show when=move || has_summary fallback=|| ()>
                <p class="mt-1 text-sm text-[color:var(--fg-muted)]">{article.summary.clone()}</p>
            </Show>
        </li>
    }
}

/// `/articles/:slug` — 文章详情（Markdown 在服务端渲染）。
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
                    let body_html = detail.body_html.clone();
                    view! {
                        // 详情页用 page-panel 套上 0.30 玻璃面，markdown 内容
                        // 落在稳定的"纸面"上而不是直接压到背景渐变。
                        <article class="page-panel mx-auto my-8 max-w-4xl px-4">
                            <p class="mb-4">
                                <A href="/articles" attr:class="link-text text-sm">
                                    "← 文章"
                                </A>
                            </p>
                            // 详情页保留 h1（这是文章标题本身，不是 page chrome）、
                            // 发布日期。markdown-body 在 translucent bg 上展示。
                            <h1 class="page-title mb-4">{title}</h1>
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
