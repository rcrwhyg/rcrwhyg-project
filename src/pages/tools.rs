use leptos::prelude::*;
use leptos_router::components::A;

use crate::server::{db_status, server_ping};
use crate::tools::registry::all_tools;

#[component]
pub fn ToolsIndexPage() -> impl IntoView {
    let tools = all_tools();

    view! {
        <section class="page-panel mx-auto my-8 max-w-3xl px-4">
            <h1 class="page-title mb-2">"工具集合"</h1>
            <p class="mb-8 text-[color:var(--fg-muted)]">
                "新工具通过 tools::registry 登记。点击进入占位页。"
            </p>
            <ul class="grid gap-4 sm:grid-cols-2">
                {tools
                    .into_iter()
                    .map(|tool| {
                        view! {
                            <li class="border border-[color:var(--border)] bg-[color:var(--surface)] p-4">
                                <h2 class="text-[color:var(--accent)]">{tool.title}</h2>
                                <p class="mt-2 text-sm text-[color:var(--fg-muted)]">{tool.summary}</p>
                                <A
                                    href=tool.path
                                    attr:class="mt-3 inline-block text-sm text-[color:var(--link)]"
                                >
                                    "打开 →"
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
        <section class="page-panel mx-auto my-8 max-w-3xl px-4">
            <h1 class="page-title mb-4">"echo · 占位工具"</h1>
            <p class="text-[color:var(--fg-muted)]">
                "演示 Resource + Suspense、可选 Postgres context，以及 SSE / WebSocket 占位端点。"
            </p>

            <div class="mt-6 space-y-4">
                <div class="border border-[color:var(--border)] p-4">
                    <p class="mb-2 text-sm text-[color:var(--accent)]">"Suspense / server_ping"</p>
                    <Suspense fallback=move || {
                        view! { <p class="text-[color:var(--fg-dim)]">"loading…"</p> }
                    }>
                        {move || match ping.get() {
                            Some(Ok(msg)) => view! { <p>{msg}</p> }.into_any(),
                            Some(Err(err)) => {
                                view! { <p class="text-[color:var(--danger)]">{err.to_string()}</p> }
                                    .into_any()
                            }
                            None => view! { <p>"…"</p> }.into_any(),
                        }}
                    </Suspense>
                </div>

                <div class="border border-[color:var(--border)] p-4">
                    <p class="mb-2 text-sm text-[color:var(--accent)]">"Suspense / db_status"</p>
                    <Suspense fallback=move || {
                        view! { <p class="text-[color:var(--fg-dim)]">"checking db…"</p> }
                    }>
                        {move || match db.get() {
                            Some(Ok(msg)) => view! { <p>{msg}</p> }.into_any(),
                            Some(Err(err)) => {
                                view! { <p class="text-[color:var(--danger)]">{err.to_string()}</p> }
                                    .into_any()
                            }
                            None => view! { <p>"…"</p> }.into_any(),
                        }}
                    </Suspense>
                </div>
            </div>

            <ul class="mt-4 space-y-1 text-sm text-[color:var(--fg-dim)]">
                <li>
                    "HTTP health："
                    <a href="/health" class="text-[color:var(--link)]" target="_blank" rel="noreferrer">
                        "/health"
                    </a>
                </li>
                <li>
                    "SSE："
                    <a
                        href="/sse/heartbeat"
                        class="text-[color:var(--link)]"
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
