use leptos::prelude::*;
use leptos_router::components::{A, Redirect};
use leptos_router::hooks::use_navigate;
use leptos_router::NavigateOptions;

use crate::app::{clear_admin_session, set_logged_in_admin, use_admin_session};
use crate::domain::AdminPublic;
use crate::server::{admin_bootstrap_status, admin_login, admin_logout};

#[component]
pub fn AdminGate() -> impl IntoView {
    let status = Resource::new(|| (), |_| async move { admin_bootstrap_status().await });

    view! {
        <section class="page-panel mx-auto my-8 max-w-lg px-4">
            <Suspense fallback=move || {
                view! { <p class="text-[color:var(--fg-dim)]">"检查登录状态…"</p> }
            }>
                {move || match status.get() {
                    None => view! { <p class="text-[color:var(--fg-dim)]">"检查登录状态…"</p> }
                        .into_any(),
                    Some(Err(err)) => view! {
                        <div class="space-y-3">
                            <h1 class="page-title">"后台"</h1>
                            <p class="text-[color:var(--danger)]">{format!("{err}")}</p>
                            <p class="text-sm text-[color:var(--fg-muted)]">
                                "请确认 DATABASE_URL，并执行 sql/auth.sql。"
                            </p>
                        </div>
                    }
                    .into_any(),
                    Some(Ok(s)) if !s.has_admin => {
                        view! { <AdminBootstrapHint /> }.into_any()
                    }
                    Some(Ok(s)) if !s.logged_in => {
                        view! { <Redirect path="/admin/login" /> }.into_any()
                    }
                    Some(Ok(s)) => match s.admin {
                        Some(admin) => view! { <AdminDashboard admin=admin /> }.into_any(),
                        None => view! { <Redirect path="/admin/login" /> }.into_any(),
                    },
                }}
            </Suspense>
        </section>
    }
}

#[component]
fn AdminBootstrapHint() -> impl IntoView {
    view! {
        <div class="space-y-4">
            <h1 class="page-title">"尚未初始化管理员"</h1>
            <p class="text-sm text-[color:var(--fg-muted)]">
                "出于安全考虑，管理员只能在服务器本机用 CLI 创建，浏览器不再提供公开初始化入口。"
            </p>
            <pre class="overflow-x-auto rounded border border-[color:var(--border)] bg-[color:var(--bg-elevated)] p-3 text-xs text-[color:var(--fg)]">
                {"cargo run --features ssr --bin create-admin -- you@example.com '至少12位密码'"}
            </pre>
            <p class="text-sm text-[color:var(--fg-dim)]">
                "创建后访问 "
                <A href="/admin/login" attr:class="underline">"/admin/login"</A>
                " 登录。"
            </p>
        </div>
    }
}

#[component]
fn AdminDashboard(admin: AdminPublic) -> impl IntoView {
    let logout_pending = RwSignal::new(false);
    let navigate = use_navigate();
    let session = use_admin_session();
    let email = admin.email.clone();

    view! {
        <div class="space-y-4">
            <h1 class="page-title">"管理后台"</h1>
            <p class="text-[color:var(--fg-muted)]">
                {format!("已登录：{email}")}
            </p>
            <p class="text-sm text-[color:var(--fg-dim)]">
                "顶栏在登录后会显示「后台 / 文章」入口，浏览前台时也可随时返回。"
            </p>
            <div class="flex flex-wrap gap-3">
                <A href="/admin/posts" attr:class="control-btn">"文章管理"</A>
                <A href="/blog" attr:class="control-btn">"查看博客"</A>
                <button
                    type="button"
                    class="control-btn"
                    disabled=move || logout_pending.get()
                    on:click=move |_| {
                        logout_pending.set(true);
                        let navigate = navigate.clone();
                        leptos::task::spawn_local(async move {
                            let _ = admin_logout().await;
                            clear_admin_session(session);
                            navigate("/", NavigateOptions::default());
                        });
                    }
                >
                    "退出登录"
                </button>
            </div>
        </div>
    }
}

#[component]
pub fn AdminLoginPage() -> impl IntoView {
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let error = RwSignal::new(Option::<String>::None);
    let pending = RwSignal::new(false);
    let navigate = use_navigate();
    let session = use_admin_session();
    let status = Resource::new(|| (), |_| async move { admin_bootstrap_status().await });

    view! {
        <section class="page-panel mx-auto my-8 max-w-lg px-4">
            <Suspense fallback=|| ()>
                {move || match status.get() {
                    Some(Ok(s)) if !s.has_admin => {
                        view! {
                            <div class="mb-6 space-y-3">
                                <h1 class="page-title">"尚未初始化管理员"</h1>
                                <pre class="overflow-x-auto rounded border border-[color:var(--border)] bg-[color:var(--bg-elevated)] p-3 text-xs">
                                    {"cargo run --features ssr --bin create-admin -- you@example.com '至少12位密码'"}
                                </pre>
                            </div>
                        }
                        .into_any()
                    }
                    Some(Ok(s)) if s.logged_in => {
                        view! { <Redirect path="/admin" /> }.into_any()
                    }
                    _ => ().into_any(),
                }}
            </Suspense>
            <h1 class="page-title mb-4">"管理员登录"</h1>
            <form
                class="space-y-4"
                on:submit=move |ev| {
                    ev.prevent_default();
                    error.set(None);
                    pending.set(true);
                    let email_v = email.get_untracked();
                    let password_v = password.get_untracked();
                    let navigate = navigate.clone();
                    leptos::task::spawn_local(async move {
                        match admin_login(email_v, password_v).await {
                            Ok(admin) => {
                                set_logged_in_admin(session, admin);
                                navigate("/admin", NavigateOptions::default());
                            }
                            Err(err) => {
                                error.set(Some(err.to_string()));
                                pending.set(false);
                            }
                        }
                    });
                }
            >
                <label class="block space-y-1 text-sm">
                    <span class="text-[color:var(--fg-muted)]">"邮箱"</span>
                    <input
                        class="control-btn w-full"
                        type="email"
                        required
                        prop:value=move || email.get()
                        on:input=move |ev| email.set(event_target_value(&ev))
                    />
                </label>
                <label class="block space-y-1 text-sm">
                    <span class="text-[color:var(--fg-muted)]">"密码"</span>
                    <input
                        class="control-btn w-full"
                        type="password"
                        required
                        minlength="12"
                        prop:value=move || password.get()
                        on:input=move |ev| password.set(event_target_value(&ev))
                    />
                </label>
                <Show when=move || error.get().is_some() fallback=|| ()>
                    <p class="text-sm text-[color:var(--danger)]">{error.get().unwrap_or_default()}</p>
                </Show>
                <button type="submit" class="control-btn" disabled=move || pending.get()>
                    {move || if pending.get() { "登录中…" } else { "登录" }}
                </button>
            </form>
        </section>
    }
}
