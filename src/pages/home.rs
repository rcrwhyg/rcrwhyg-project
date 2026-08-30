use leptos::prelude::*;
use leptos_router::components::A;

use crate::server::{RecentItem, recent_items};

#[component]
pub fn HomePage() -> impl IntoView {
    let recent = Resource::new(|| (), |_| async move { recent_items().await });

    view! {
        <div class="mx-auto my-12 max-w-4xl space-y-10 px-4">
            <SectionCarousel />
            <section>
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
            <div class="section-carousel">
                <div class="section-carousel__track">
                    {ITEMS
                        .iter()
                        .map(|item| {
                            view! {
                                <A href=item.href attr:class=section_card_class(item.cls)>
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
                            view! {
                                <A
                                    href=item.href
                                    attr:class=section_card_class(item.cls)
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

fn section_card_class(cls: &str) -> &'static str {
    match cls {
        "s-sky" => "section-card s-sky",
        "s-mint" => "section-card s-mint",
        "s-mix" => "section-card s-mix",
        _ => "section-card",
    }
}

#[component]
fn RecentRow(item: RecentItem) -> impl IntoView {
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
                        attr:class="no-underline text-[color:var(--fg)] hover:text-[color:var(--fg-muted)]"
                    >
                        {title}
                    </A>
                </h2>
                <span class="tag">"文章"</span>
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
