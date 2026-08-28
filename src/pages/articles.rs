use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;

use crate::server::{ArticleMeta, get_site_article, list_site_articles};

/// `/articles` — 个人网站首发文章索引（由 articles/*.md 生成）。
#[component]
pub fn ArticlesIndexPage() -> impl IntoView {
    let articles = Resource::new(|| (), |_| async move { list_site_articles().await });

    view! {
        <section class="page-panel mx-auto my-8 max-w-3xl px-4">
            <h1 class="page-title mb-2">"文章"</h1>
            <p class="mb-8 text-[color:var(--fg-muted)]">
                "个人网站首发文章，与微信公众号【如春日午后阳光】的转载对应。"
            </p>
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
                        <ul class="space-y-4">
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
        <li class="border border-[color:var(--border)] bg-[color:var(--surface)]/80 p-4">
            <h2>
                <A href=href.clone() attr:class="no-underline text-[color:var(--accent)] hover:text-[color:var(--link)]">
                    {article.title.clone()}
                </A>
            </h2>
            <Show when=move || has_date fallback=|| ()>
                <p class="mt-2 text-sm text-[color:var(--fg-dim)]">{date.clone()}</p>
            </Show>
            <Show when=move || has_summary fallback=|| ()>
                <p class="mt-2 text-sm text-[color:var(--fg-muted)]">{article.summary.clone()}</p>
            </Show>
            <A href=href attr:class="mt-3 inline-block text-sm text-[color:var(--link)]">
                "阅读全文 →"
            </A>
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
                    <section class="page-panel mx-auto my-8 max-w-3xl px-4">
                        <p class="text-[color:var(--fg-dim)]">"加载文章…"</p>
                    </section>
                }
                .into_any(),
                Some(Err(err)) => view! {
                    <section class="page-panel mx-auto my-8 max-w-3xl px-4">
                        <p class="text-[color:var(--danger)]">{format!("加载失败：{err}")}</p>
                        <A href="/articles" attr:class="mt-4 inline-block text-sm text-[color:var(--link)]">
                            "返回文章"
                        </A>
                    </section>
                }
                .into_any(),
                Some(Ok(None)) => view! {
                    <section class="page-panel mx-auto my-8 max-w-3xl px-4">
                        <h1 class="page-title mb-4">"未找到文章"</h1>
                        <A href="/articles" attr:class="text-sm text-[color:var(--link)]">
                            "返回文章"
                        </A>
                    </section>
                }
                .into_any(),
                Some(Ok(Some(detail))) => {
                    let title = detail.meta.title.clone();
                    let meta = detail.meta.date.clone().unwrap_or_else(|| "未标注日期".into());
                    let body_html = detail.body_html.clone();
                    view! {
                        <article class="page-panel mx-auto my-8 max-w-3xl px-4">
                            <p class="mb-4">
                                <A href="/articles" attr:class="text-sm text-[color:var(--link)]">
                                    "← 文章"
                                </A>
                            </p>
                            <h1 class="page-title mb-4">{title}</h1>
                            <p class="mb-8 text-sm text-[color:var(--fg-dim)]">{format!("发布于 {meta}")}</p>
                            <div class="markdown-body" inner_html=body_html></div>
                        </article>
                    }
                    .into_any()
                }
            }}
        </Suspense>
    }
}
