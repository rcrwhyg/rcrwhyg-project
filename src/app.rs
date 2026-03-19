use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/rcrwhyg-server.css"/>

        // sets the document title
        <Title text="如春日午后阳光"/>

        // content for this welcome page
        <Router>
            // 极简布局：主内容区 + 底部 Footer
            <div class="min-h-screen flex flex-col bg-gray-50 text-gray-800 font-sans">
                // 顶部导航（装装样子，让网站看起来完整）
                <header class="bg-white shadow-sm py-4">
                    <div class="max-w-3xl mx-auto px-4 flex justify-between items-center">
                        <h1 class="text-xl font-bold tracking-tight">如春日午后阳光</h1>
                        <nav class="text-sm text-gray-500">"Home"</nav>
                    </div>
                </header>

                <main>
                    <Routes fallback=|| "Page not found.".into_view()>
                        <Route path=StaticSegment("") view=HomePage/>
                    </Routes>
                </main>

                // --- 备案审核最关键的部分 ---
                <footer class="bg-white border-t py-8 mt-auto">
                    <div class="max-w-3xl mx-auto px-4 text-center text-sm text-gray-500 space-y-2">
                        <p>"Copyright © 2026. All Rights Reserved."</p>
                        <p>
                            // ⚠️ 请务必将这里替换为你的真实 ICP 备案号！
                            "备案号："
                            <a
                                href="https://beian.miit.gov.cn/"
                                target="_blank"
                                class="hover:text-blue-600 transition-colors"
                            >
                                "陕ICP备2026003918号-1"
                            </a>
                        </p>
                    </div>
                </footer>
            </div>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <article class="bg-white p-6 md:p-10 rounded-lg shadow-sm">
            <h2 class="text-2xl md:text-3xl font-bold mb-4 text-gray-900">
                "朴素的启程：写在个人网站上线之前"
            </h2>
            <div class="text-gray-400 text-sm mb-8 border-b pb-4">
                "发布时间：2026年3月"
            </div>

            // 正文内容，简单排版
            <div class="space-y-4 leading-relaxed text-gray-700">
                <p>"大家好，很高兴在这里开启我的第一次分享。"</p>

                <p>"起因很简单：我手上有一台 2 核 2G 配置的轻量云服务器。在这个动辄微服务、分布式的时代，它显得非常局促。但如果放任它吃灰，未免有些可惜。"</p>

                <p>"于是我决定回归初心，动手给自己搭建一个完全属于自己的个人网站，并以此为契机，把日常的学习、踩坑和技术思考记录下来。"</p>

                <h3 class="text-xl font-semibold mt-6 mb-2 text-gray-900">"技术选型"</h3>
                <p>"作为喜欢折腾的开发者，我选择了 Rust + Leptos 作为技术栈。Rust 极低的内存占用非常适合我的 2C2G 服务器，而 Leptos 的服务端渲染（SSR）能带来极佳的首屏加载体验。"</p>

                <p>"这是一个关于学习与记录、成长与分享的角落。目前网站的基础脚手架刚刚搭建完毕，正在进行公安联网备案。"</p>

                <p class="font-medium mt-8 text-gray-900">"准备好了，我们这就出发。"</p>
            </div>
        </article>
    }
}
