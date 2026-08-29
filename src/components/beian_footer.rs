use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn BeianFooter() -> impl IntoView {
    view! {
        <footer class="site-footer">
            <div class="mx-auto max-w-5xl space-y-3 px-4 text-center text-sm muted-text">
                <p>"Copyright © 2026. All Rights Reserved."</p>
                <div class="flex flex-wrap items-center justify-center gap-4">
                    <a
                        href="https://beian.miit.gov.cn/"
                        target="_blank"
                        rel="noreferrer"
                        class="link-text hover:text-[color:var(--accent)]"
                    >
                        "陕ICP备2026003918号-1"
                    </a>
                    <a
                        href="https://beian.mps.gov.cn/#/query/webSearch?code=61011302002365"
                        target="_blank"
                        rel="noreferrer"
                        class="inline-flex items-center gap-2 link-text hover:text-[color:var(--accent)]"
                    >
                        <img src="/beian.png" alt="公安备案图标" class="h-4 w-4" />
                        "陕公网安备61011302002365号"
                    </a>
                </div>
                <p class="text-xs dim-text">
                    "轻量工具："
                    <A href="/music" attr:class="link-text mx-1">"音乐"</A>
                    <span aria-hidden="true">"·"</span>
                    <A href="/clock" attr:class="link-text mx-1">"番茄钟"</A>
                </p>
            </div>
        </footer>
    }
}
