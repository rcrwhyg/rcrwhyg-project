use leptos::prelude::*;
use leptos_router::components::A;

use crate::server::{RecentItem, recent_items};

/// `/` — hub. 板块入口用无限横向滚动轮播 + 最近更新 + 顶/底留白
/// 都重做了一遍（issues 1-8 反馈的落实）。原 hub-hero + tool-footer
/// 删了；板块卡片在 CSS-only 无限轮播里循环（hover 暂停）。
#[component]
pub fn HomePage() -> impl IntoView {
    let recent = Resource::new(|| (), |_| async move { recent_items().await });

    view! {
        <div class="mx-auto my-12 max-w-4xl space-y-10 px-4">
            <SectionCarousel />
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
                            <ul class="space-y-5">
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
        </div>
    }
}

/// 板块横向无限轮播。track 里塞 2 份同样的 5 张 section-card
/// （CSS 关键帧把 track translateX 从 0 滚到 -50%，正好一份
/// 原始宽度的距离，过渡到 0% 时刚好是循环起点）。
#[component]
fn SectionCarousel() -> impl IntoView {
    const ITEMS: [SectionItem; 5] = [
        SectionItem {
            href: "/articles",
            cls: "s-sky",
            eyebrow: "/articles",
            name: "文章",
            blurb: "技术 / 经验分享 · 文件式 + git+CD 自动化部署。",
        },
        SectionItem {
            href: "/tools",
            cls: "s-mint",
            eyebrow: "/tools",
            name: "工具",
            blurb: "自用小工具集 · 通过 tools::registry 登记。",
        },
        SectionItem {
            href: "/radar",
            cls: "s-sky",
            eyebrow: "/radar",
            name: "学习雷达",
            blurb: "多生态学习进度图（Rust / Java / Zig / Cangjie / MoonBit …）。",
        },
        SectionItem {
            href: "/lab",
            cls: "s-mint",
            eyebrow: "/lab",
            name: "实验室",
            blurb: "Rust / Zig 写的小游戏、demo、可视化实验。",
        },
        SectionItem {
            href: "/about",
            cls: "s-mix",
            eyebrow: "/about",
            name: "关于",
            blurb: "创刊叙事、板块速览、技术栈与联系方式。",
        },
    ];

    view! {
        <section>
            <h2 class="section-title">"板块"</h2>
            <div class="section-carousel">
                <div class="section-carousel__track">
                    {ITEMS
                        .iter()
                        .map(|item| {
                            let class = format!("section-card {}", item.cls);
                            view! {
                                <A href=item.href attr:class=class>
                                    <span class="eyebrow">{item.eyebrow}</span>
                                    <h3>{item.name}</h3>
                                    <p>{item.blurb}</p>
                                    <span class="open-link">"打开 →"</span>
                                </A>
                            }
                        })
                        .collect_view()}
                    {ITEMS
                        .iter()
                        .map(|item| {
                            let class = format!("section-card {}", item.cls);
                            view! {
                                <A
                                    href=item.href
                                    attr:class=class
                                    attr:aria-hidden="true"
                                    attr:tabindex="-1"
                                >
                                    <span class="eyebrow">{item.eyebrow}</span>
                                    <h3>{item.name}</h3>
                                    <p>{item.blurb}</p>
                                    <span class="open-link">"打开 →"</span>
                                </A>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </section>
    }
}

struct SectionItem {
    href: &'static str,
    cls: &'static str,
    eyebrow: &'static str,
    name: &'static str,
    blurb: &'static str,
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
                    <A
                        href=item.href
                        attr:class="no-underline text-[color:var(--accent)] hover:text-[color:var(--link)]"
                    >
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
