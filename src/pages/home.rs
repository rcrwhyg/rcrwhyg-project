use leptos::prelude::*;
use leptos_router::components::A;

use crate::server::{RecentItem, recent_items};

/// `/` — hub. Replaces the original "创始文" home with a section-card grid
/// + recent updates. Old narrative lives in `content/about.md`.
#[component]
pub fn HomePage() -> impl IntoView {
    let recent = Resource::new(|| (), |_| async move { recent_items().await });

    view! {
        <div class="mx-auto my-8 max-w-5xl space-y-8 px-4">
            <HubHero />
            <SectionGrid />
            <section>
                <h2 class="section-title">"最近更新"</h2>
                <Suspense fallback=move || {
                    view! { <p class="dim-text">"加载中…"</p> }
                }>
                    {move || match recent.get() {
                        None => view! { <p class="dim-text">"加载中…"</p> }.into_any(),
                        Some(Err(err)) => view! {
                            <p class="danger-text">{format!("加载失败：{err}")}</p>
                        }
                        .into_any(),
                        Some(Ok(items)) if items.is_empty() => view! {
                            <p class="dim-text">"暂无内容。"</p>
                        }
                        .into_any(),
                        Some(Ok(items)) => view! {
                            <ul class="space-y-3">
                                {items
                                    .into_iter()
                                    .map(|item| view! { <RecentRow item=item /> })
                                    .collect_view()}
                            </ul>
                        }
                        .into_any(),
                    }}
                </Suspense>
            </section>
            <ToolFooter />
        </div>
    }
}

#[component]
fn HubHero() -> impl IntoView {
    view! {
        <section class="hub-hero">
            <h1>
                "在 "
                <span class="mint">"薄荷"</span>
                " 与 "
                <span class="sky">"天空"</span>
                " 之间，写下学习与作品"
            </h1>
            <p class="lead">
                "「如春日午后阳光」 — 个人站点枢纽。Java / Rust 主线，React 副线；Zig / Cangjie / MoonBit 多元学习中。技术文章、自用小工具、学习雷达、实验 demo，都在这里。"
            </p>
            <div class="meta-row">
                <span>"建于 2026-03"</span>
                <span aria-hidden="true">"·"</span>
                <span>"git + CD 驱动"</span>
                <span aria-hidden="true">"·"</span>
                <span>"Leptos 0.8 · Axum 0.8 · Rust"</span>
            </div>
        </section>
    }
}

#[component]
fn SectionGrid() -> impl IntoView {
    view! {
        <section>
            <h2 class="section-title">"板块"</h2>
            <div class="section-grid">
                <A href="/articles" attr:class="section-card s-sky">
                    <span class="eyebrow">"/articles"</span>
                    <h3>"文章"</h3>
                    <p>"技术 / 经验分享 · 文件式 + git+CD 自动化部署。"</p>
                    <span class="open-link">"浏览全部 →"</span>
                </A>
                <A href="/tools" attr:class="section-card s-mint">
                    <span class="eyebrow">"/tools"</span>
                    <h3>"工具"</h3>
                    <p>"自用小工具集 · 通过 tools::registry 登记。"</p>
                    <span class="open-link">"打开 →"</span>
                </A>
                <A href="/radar" attr:class="section-card s-sky">
                    <span class="eyebrow">"/radar"</span>
                    <h3>"学习雷达"</h3>
                    <p>"多生态学习进度图（Rust / Java / Zig / Cangjie / MoonBit …）。"</p>
                    <span class="open-link">"查看 →"</span>
                </A>
                <A href="/lab" attr:class="section-card s-mint">
                    <span class="eyebrow">"/lab"</span>
                    <h3>"实验室"</h3>
                    <p>"Rust / Zig 写的小游戏、demo、可视化实验。"</p>
                    <span class="open-link">"逛逛 →"</span>
                </A>
                <A href="/about" attr:class="section-card s-mix">
                    <span class="eyebrow">"/about"</span>
                    <h3>"关于"</h3>
                    <p>"创刊叙事、板块速览、技术栈与联系方式。"</p>
                    <span class="open-link">"了解更多 →"</span>
                </A>
            </div>
        </section>
    }
}

#[component]
fn RecentRow(item: RecentItem) -> impl IntoView {
    let kind_label = if item.kind == "article" {
        "文章"
    } else {
        "工具"
    };
    let kind_class = if item.kind == "article" {
        "tag"
    } else {
        "tag sky"
    };
    let title = item.title.clone();
    let summary = item.summary.clone().unwrap_or_default();
    let has_summary = !summary.is_empty();
    let date = item.date.clone().unwrap_or_default();
    let has_date = !date.is_empty();

    view! {
        <li class="list-card s-sky">
            <div class="flex flex-wrap items-baseline justify-between gap-3">
                <h2 class="text-base font-semibold">
                    <A href=item.href attr:class="no-underline text-[color:var(--accent)] hover:text-[color:var(--link)]">
                        {title}
                    </A>
                </h2>
                <span class=kind_class>{kind_label}</span>
            </div>
            <Show when=move || has_date fallback=|| ()>
                <p class="mt-1 text-sm dim-text font-mono">{date.clone()}</p>
            </Show>
            <Show when=move || has_summary fallback=|| ()>
                <p class="mt-1 text-sm muted-text">{summary.clone()}</p>
            </Show>
        </li>
    }
}

#[component]
fn ToolFooter() -> impl IntoView {
    view! {
        <section class="text-center">
            <p class="dim-text text-sm">
                "轻量工具："
                <A href="/music" attr:class="link-text mx-1">"音乐"</A>
                <span aria-hidden="true">"·"</span>
                <A href="/clock" attr:class="link-text mx-1">"番茄钟"</A>
            </p>
        </section>
    }
}
