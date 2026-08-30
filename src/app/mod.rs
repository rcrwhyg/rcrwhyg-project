mod admin_session;
mod dynamic_bg;
mod layout;
mod theme;

pub use admin_session::{
    AdminSession, clear_admin_session, provide_admin_session, set_logged_in_admin,
    use_admin_session,
};
pub use dynamic_bg::DynamicBackground;
pub use layout::{SiteHeader, SiteShell};
pub use theme::{SitePreference, provide_site_preference, use_site_preference};

use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
use thaw::ConfigProvider;
use thaw::ssr::SSRMountStyleProvider;

use crate::pages::{
    AboutPage, AdminGate, AdminLoginPage, ArticlePage, ArticlesIndexPage, ClockPage, HomePage,
    LabPage, MusicPage, NotFoundPage, RadarPage, ToolsIndexPage, ToolsPlaceholderPage,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <SSRMountStyleProvider>
            <!DOCTYPE html>
            <html lang="zh-CN" data-theme="dark">
                <head>
                    <meta charset="utf-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1" />
                    <link rel="preconnect" href="https://fonts.googleapis.com" />
                    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
                    <link
                        href="https://fonts.googleapis.com/css2?family=Noto+Sans+SC:wght@400;500;700&family=Space+Grotesk:wght@500;700&family=JetBrains+Mono:wght@400;600&display=swap"
                        rel="stylesheet"
                    />
                    <AutoReload options=options.clone() />
                    <HydrationScripts options />
                    <MetaTags />
                </head>
                <body data-theme="dark">
                    <App />
                </body>
            </html>
        </SSRMountStyleProvider>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_site_preference();
    provide_admin_session();
    let preference = use_site_preference();

    view! {
        <ConfigProvider theme=preference.thaw_theme>
            <Stylesheet id="leptos" href="/pkg/rcrwhyg-server.css" />
            <Title text="如春日午后阳光" />
            <Router>
                <SiteShell>
                    <Routes fallback=|| view! { <NotFoundPage /> }.into_any()>
                        <Route path=path!("") view=HomePage />
                        <Route path=path!("articles") view=ArticlesIndexPage />
                        <Route path=path!("articles/:slug") view=ArticlePage />
                        <Route path=path!("tools") view=ToolsIndexPage />
                        <Route path=path!("tools/echo") view=ToolsPlaceholderPage />
                        <Route path=path!("radar") view=RadarPage />
                        <Route path=path!("lab") view=LabPage />
                        <Route path=path!("about") view=AboutPage />
                        <Route path=path!("music") view=MusicPage />
                        <Route path=path!("clock") view=ClockPage />
                        <Route path=path!("admin") view=AdminGate />
                        <Route path=path!("admin/login") view=AdminLoginPage />
                    </Routes>
                </SiteShell>
            </Router>
        </ConfigProvider>
    }
}
