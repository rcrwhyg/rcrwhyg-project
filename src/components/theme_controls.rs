use leptos::prelude::*;

use crate::app::{ThemeMode, use_site_preference};

/// 主题切换按钮。**目标是用 leptos 完成**——vanilla JS 兜底是临时替代，
/// 不应成为终态。正确 leptos 写法（见 `rules/leptos-ssr-hydrate.md` 对写法 C）：
/// - 闭包 `on_click` 零 cfg，签名 SSR/hydrate 一致
/// - DOM 操作放 cfg-整块的辅助函数 `apply_theme_hydrate`
/// - Effect 调辅助函数（hydrated 时跑，SSR 时辅助函数不存在 → no-op）
#[component]
pub fn ThemeControls() -> impl IntoView {
    let preference = use_site_preference();

    // 闭包只动 RwSignal，零 cfg
    let on_click = move || {
        preference.set_theme.update(|t| *t = t.toggle());
    };

    // Effect 调辅助函数
    Effect::new(move |_| {
        apply_theme(preference.theme.get());
    });

    view! {
        <div class="theme-controls flex shrink-0 items-center gap-2">
            <button
                type="button"
                class="control-btn"
                on:click={
                    move |_: leptos::ev::MouseEvent| {
                        on_click();
                    }
                }
            >
                {move || match preference.theme.get() {
                    ThemeMode::Dark => "切换亮色",
                    ThemeMode::Light => "切换暗色",
                }}
            </button>
        </div>
    }
}

/// cfg 整块——SSR 编译期函数不存在（no-op），hydrate 编译期函数存在。
/// Effect 调它：SSR no-op，hydrate 实际写 DOM。
#[allow(unused_variables)]
fn apply_theme(theme: ThemeMode) {
    #[cfg(feature = "hydrate")]
    {
        apply_theme_hydrate(theme);
    }
}

#[cfg(feature = "hydrate")]
fn apply_theme_hydrate(theme: ThemeMode) {
    let value = theme.as_str();
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.set_item("rcrwhyg.theme", value);
        }
        if let Some(document) = window.document() {
            if let Some(html) = document.document_element() {
                let _ = html.set_attribute("data-theme", value);
            }
            if let Some(body) = document.body() {
                let _ = body.set_attribute("data-theme", value);
            }
        }
    }
}
