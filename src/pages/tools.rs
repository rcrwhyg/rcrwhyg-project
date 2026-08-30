use leptos::prelude::*;
use leptos_router::components::A;

use crate::server::{db_status, server_ping};
use crate::tools::registry::all_tools;

/// `/tools` — 工具占位卡片列表。按反馈：保留卡片作为占位、移除
/// 名称和介绍，背景更透明、能看到星星。
#[component]
pub fn ToolsIndexPage() -> impl IntoView {
    let tools = all_tools();

    view! {
        <section class="mx-auto my-8 max-w-4xl px-4">
            <ul class="grid gap-4 sm:grid-cols-2 md:grid-cols-3">
                {tools
                    .into_iter()
                    .map(|tool| {
                        view! {
                            <li class="list-card flex items-center justify-center min-h-64">
                                <A
                                    href=tool.path
                                    attr:class="link-text text-sm"
                                >
                                    "示例工具 →"
                                </A>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </section>
    }
}

#[component]
pub fn ToolsPlaceholderPage() -> impl IntoView {
    let ping = Resource::new(|| (), |_| async move { server_ping().await });
    let db = Resource::new(|| (), |_| async move { db_status().await });

    view! {
        // 工具详情：用 page-panel 拿 0.30 玻璃面，文本落到稳定纸面
        <section class="page-panel mx-auto my-8 max-w-4xl px-4">
            <p class="dim-text">
                "演示 Resource + Suspense、可选 Postgres context，以及 SSE / WebSocket 占位端点。"
            </p>

            <div class="mt-6 space-y-4">
                <div class="list-card">
                    <p class="mb-2 text-sm dim-text">"Suspense / server_ping"</p>
                    <Suspense fallback=move || {
                        view! { <p class="dim-text">"loading…"</p> }
                    }>
                        {move || match ping.get() {
                            Some(Ok(msg)) => view! { <p>{msg}</p> }.into_any(),
                            Some(Err(err)) => {
                                view! { <p class="danger-text">{err.to_string()}</p> }
                                    .into_any()
                            }
                            None => view! { <p>"…"</p> }.into_any(),
                        }}
                    </Suspense>
                </div>

                <div class="list-card">
                    <p class="mb-2 text-sm dim-text">"Suspense / db_status"</p>
                    <Suspense fallback=move || {
                        view! { <p class="dim-text">"checking db…"</p> }
                    }>
                        {move || match db.get() {
                            Some(Ok(msg)) => view! { <p>{msg}</p> }.into_any(),
                            Some(Err(err)) => {
                                view! { <p class="danger-text">{err.to_string()}</p> }
                                    .into_any()
                            }
                            None => view! { <p>"…"</p> }.into_any(),
                        }}
                    </Suspense>
                </div>
            </div>

            <ul class="mt-4 space-y-1 text-sm dim-text">
                <li>
                    "HTTP health："
                    <a href="/health" class="link-text" target="_blank" rel="noreferrer">
                        "/health"
                    </a>
                </li>
                <li>
                    "SSE："
                    <a
                        href="/sse/heartbeat"
                        class="link-text"
                        target="_blank"
                        rel="noreferrer"
                    >
                        "/sse/heartbeat"
                    </a>
                </li>
                <li>"WebSocket：""/ws/echo""（可用 websocat / 浏览器 WS 客户端连接）"</li>
            </ul>

            <A href="/tools" attr:class="control-btn mt-6 inline-block">
                "← 返回工具列表"
            </A>
        </section>
    }
}
