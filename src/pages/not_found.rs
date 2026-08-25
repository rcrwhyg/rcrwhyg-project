use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <section class="page-panel mx-auto my-16 max-w-xl px-4 text-center">
            <h1 class="page-title mb-4">"404"</h1>
            <p class="text-[color:var(--fg-muted)]">"页面不存在，或路径尚未接线。"</p>
            <A href="/" attr:class="control-btn mt-6 inline-block">"返回首页"</A>
        </section>
    }
}
