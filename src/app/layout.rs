use leptos::prelude::*;
use leptos_router::NavigateOptions;
use leptos_router::components::A;
use leptos_router::hooks::use_navigate;

use crate::app::DynamicBackground;
use crate::app::{clear_admin_session, use_admin_session, use_site_preference};
use crate::components::{BeianFooter, ThemeControls};
use crate::server::admin_logout;

#[component]
pub fn SiteHeader() -> impl IntoView {
    let session = use_admin_session();
    let logged_in = Memo::new(move |_| session.admin.get().is_some());
    let logout_pending = RwSignal::new(false);
    let navigate = use_navigate();

    view! {
        <header class="site-header glow-edge">
            <div class="site-header-inner">
                <div class="flex min-w-0 items-center gap-4">
                    <A href="/" attr:class="brand-title shrink-0 text-[color:var(--accent)] no-underline">
                        "如春日午后阳光"
                    </A>
                    <nav class="hidden items-center gap-4 text-sm sm:flex">
                        <A href="/" attr:class="nav-link">"首页"</A>
                        <A href="/blog" attr:class="nav-link">"博客"</A>
                        <A href="/articles" attr:class="nav-link">"文章"</A>
                        <A href="/tools" attr:class="nav-link">"工具"</A>
                        <Show when=move || logged_in.get() fallback=|| ()>
                            <span class="text-[color:var(--fg-dim)]" aria-hidden="true">"|"</span>
                            <A href="/admin" attr:class="nav-link nav-link-admin">"后台"</A>
                            <A href="/admin/posts" attr:class="nav-link nav-link-admin">"文章"</A>
                        </Show>
                    </nav>
                </div>
                <div class="flex shrink-0 items-center gap-2 sm:gap-3">
                    <Show when=move || logged_in.get() fallback=|| ()>
                        <A href="/admin" attr:class="nav-link nav-link-admin text-sm sm:hidden">
                            "后台"
                        </A>
                        <button
                            type="button"
                            class="nav-link nav-link-admin text-sm"
                            disabled=move || logout_pending.get()
                            on:click={
                                let navigate = navigate.clone();
                                let session = session;
                                move |_| {
                                    logout_pending.set(true);
                                    let navigate = navigate.clone();
                                    leptos::task::spawn_local(async move {
                                        let _ = admin_logout().await;
                                        clear_admin_session(session);
                                        logout_pending.set(false);
                                        navigate("/", NavigateOptions::default());
                                    });
                                }
                            }
                        >
                            "退出"
                        </button>
                    </Show>
                    <ThemeControls />
                </div>
            </div>
        </header>
    }
}

#[component]
pub fn SiteShell(children: Children) -> impl IntoView {
    let preference = use_site_preference();

    view! {
        <div
            class="site-root min-h-screen"
            data-theme=move || preference.theme.get().as_str()
        >
            <DynamicBackground />
            <SiteHeader />
            <div class="site-body">
                <main class="site-main">
                    {children()}
                    <BeianFooter />
                </main>
            </div>
        </div>
    }
}
