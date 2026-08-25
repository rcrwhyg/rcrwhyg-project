use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <article class="page-panel mx-auto my-8 max-w-3xl px-4">
            <h1 class="page-title mb-4">"朴素的启程：写在个人网站上线之前"</h1>
            <p class="mb-8 text-sm text-[color:var(--fg-dim)]">"发布时间：2026年3月"</p>
            <div class="prose-cyber space-y-4">
                <p>"大家好，很高兴在这里开启我的第一次分享。"</p>
                <p>
                    "起因很简单：我手上有一台 2 核 2G 配置的轻量云服务器。在这个动辄微服务、分布式的时代，它显得非常局促。但如果放任它吃灰，未免有些可惜。"
                </p>
                <p>
                    "于是我决定回归初心，动手给自己搭建一个完全属于自己的个人网站，并以此为契机，把日常的学习、踩坑和技术思考记录下来。"
                </p>
                <h2 class="section-title">"技术选型"</h2>
                <p>
                    "作为喜欢折腾的开发者，我选择了 Rust + Leptos 作为技术栈。Rust 极低的内存占用非常适合我的 2C2G 服务器，而 Leptos 的服务端渲染（SSR）能带来极佳的首屏加载体验。"
                </p>
                <p>
                    "站点采用极致赛博朋克视觉：动态背景、鼠标追随光效，并支持暗色与亮色两套主题。博客与小工具集合会逐步长出来。"
                </p>
                <p class="mt-8 font-medium text-[color:var(--accent)]">
                    "准备好了，我们这就出发。"
                </p>
                <div class="flex flex-wrap gap-3 pt-4">
                    <A href="/blog/plain-start" attr:class="control-btn">"阅读全文"</A>
                    <A href="/blog" attr:class="control-btn">"进入博客"</A>
                    <A href="/tools" attr:class="control-btn">"打开工具箱"</A>
                </div>
            </div>
        </article>
    }
}
