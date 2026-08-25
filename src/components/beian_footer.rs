use leptos::prelude::*;

#[component]
pub fn BeianFooter() -> impl IntoView {
    view! {
        <footer class="relative z-10 mt-auto border-t border-[color:var(--border)] bg-[color:var(--surface)]/80 py-8 backdrop-blur">
            <div class="mx-auto max-w-5xl space-y-3 px-4 text-center text-sm text-[color:var(--fg-muted)]">
                <p>"Copyright © 2026. All Rights Reserved."</p>
                <div class="flex flex-wrap items-center justify-center gap-4">
                    <a
                        href="https://beian.miit.gov.cn/"
                        target="_blank"
                        rel="noreferrer"
                        class="text-[color:var(--link)] hover:text-[color:var(--accent)]"
                    >
                        "陕ICP备2026003918号-1"
                    </a>
                    <a
                        href="https://beian.mps.gov.cn/#/query/webSearch?code=61011302002365"
                        target="_blank"
                        rel="noreferrer"
                        class="inline-flex items-center gap-2 text-[color:var(--link)] hover:text-[color:var(--accent)]"
                    >
                        <img src="/beian.png" alt="公安备案图标" class="h-4 w-4" />
                        "陕公网安备61011302002365号"
                    </a>
                </div>
            </div>
        </footer>
    }
}
