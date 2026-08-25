use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;

use crate::domain::PostSummary;
use crate::server::{get_post_by_slug, list_published_posts};

#[component]
pub fn BlogPage() -> impl IntoView {
    let posts = Resource::new(|| (), |_| async move { list_published_posts().await });

    view! {
        <section class="page-panel mx-auto my-8 max-w-3xl px-4">
            <h1 class="page-title mb-2">"博客"</h1>
            <p class="mb-8 text-[color:var(--fg-muted)]">
                "本站内容优先。列表来自 posts（数据库或内置种子）。"
            </p>
            <Suspense fallback=move || {
                view! { <p class="text-[color:var(--fg-dim)]">"加载文章列表…"</p> }
            }>
                {move || match posts.get() {
                    None => view! { <p class="text-[color:var(--fg-dim)]">"加载文章列表…"</p> }
                        .into_any(),
                    Some(Err(err)) => view! {
                        <p class="text-[color:var(--danger)]">
                            {format!("加载失败：{err}")}
                        </p>
                    }
                    .into_any(),
                    Some(Ok(items)) if items.is_empty() => view! {
                        <p class="text-[color:var(--fg-dim)]">"暂无已发布文章。"</p>
                    }
                    .into_any(),
                    Some(Ok(items)) => view! {
                        <ul class="space-y-4">
                            {items
                                .into_iter()
                                .map(|post| view! { <BlogListItem post=post /> })
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
fn BlogListItem(post: PostSummary) -> impl IntoView {
    let href = format!("/blog/{}", post.slug);
    let meta = {
        let date = post.published_at.clone().unwrap_or_else(|| "草稿".into());
        if post.tags.is_empty() {
            date
        } else {
            format!("{date} · {}", post.tags.join(" / "))
        }
    };
    let summary = post.summary.clone().unwrap_or_default();
    let has_summary = !summary.is_empty();

    view! {
        <li class="border border-[color:var(--border)] bg-[color:var(--surface)]/80 p-4">
            <h2>
                <A href=href.clone() attr:class="no-underline text-[color:var(--accent)] hover:text-[color:var(--link)]">
                    {post.title.clone()}
                </A>
            </h2>
            <p class="mt-2 text-sm text-[color:var(--fg-dim)]">{meta}</p>
            <Show when=move || has_summary fallback=|| ()>
                <p class="mt-2 text-sm text-[color:var(--fg-muted)]">{summary.clone()}</p>
            </Show>
            <A href=href attr:class="mt-3 inline-block text-sm text-[color:var(--link)]">
                "阅读全文 →"
            </A>
        </li>
    }
}

#[component]
pub fn BlogPostPage() -> impl IntoView {
    let params = use_params_map();
    let post = Resource::new(
        move || params.read().get("slug").unwrap_or_default(),
        |slug| async move { get_post_by_slug(slug).await },
    );

    view! {
        <Suspense fallback=move || {
            view! {
                <section class="page-panel mx-auto my-8 max-w-3xl px-4">
                    <p class="text-[color:var(--fg-dim)]">"加载文章…"</p>
                </section>
            }
        }>
            {move || match post.get() {
                None => view! {
                    <section class="page-panel mx-auto my-8 max-w-3xl px-4">
                        <p class="text-[color:var(--fg-dim)]">"加载文章…"</p>
                    </section>
                }
                .into_any(),
                Some(Err(err)) => view! {
                    <section class="page-panel mx-auto my-8 max-w-3xl px-4">
                        <p class="text-[color:var(--danger)]">{format!("加载失败：{err}")}</p>
                        <A href="/blog" attr:class="mt-4 inline-block text-sm text-[color:var(--link)]">
                            "返回博客"
                        </A>
                    </section>
                }
                .into_any(),
                Some(Ok(None)) => view! {
                    <section class="page-panel mx-auto my-8 max-w-3xl px-4">
                        <h1 class="page-title mb-4">"未找到文章"</h1>
                        <A href="/blog" attr:class="text-sm text-[color:var(--link)]">
                            "返回博客"
                        </A>
                    </section>
                }
                .into_any(),
                Some(Ok(Some(detail))) => {
                    let title = detail.post.title.clone();
                    let mut meta = detail
                        .post
                        .published_at
                        .clone()
                        .unwrap_or_else(|| "未标注日期".into());
                    if !detail.post.tags.is_empty() {
                        meta.push_str(" · ");
                        meta.push_str(&detail.post.tags.join(" / "));
                    }
                    let body_html = detail.body_html.clone();
                    view! {
                        <article class="page-panel mx-auto my-8 max-w-3xl px-4">
                            <p class="mb-4">
                                <A href="/blog" attr:class="text-sm text-[color:var(--link)]">
                                    "← 博客"
                                </A>
                            </p>
                            <h1 class="page-title mb-4">{title}</h1>
                            <p class="mb-8 text-sm text-[color:var(--fg-dim)]">
                                {format!("发布时间：{meta}")}
                            </p>
                            <div class="prose-cyber markdown-body" inner_html=body_html></div>
                        </article>
                    }
                    .into_any()
                }
            }}
        </Suspense>
    }
}
