use leptos::prelude::*;

/// Mint + sky dynamic background: orbs (薄荷/天空) + 网格 + 柔和边缘。
/// 颜色与浓度全部由 style/tokens.css 的 token 控制（深/浅主题分别取值）。
#[component]
pub fn DynamicBackground() -> impl IntoView {
    view! {
        <div class="dynamic-bg" aria-hidden="true">
            <div class="dynamic-bg__base"></div>
            <div class="dynamic-bg__grid"></div>
            <div class="dynamic-bg__orb dynamic-bg__orb--a"></div>
            <div class="dynamic-bg__orb dynamic-bg__orb--b"></div>
            <div class="dynamic-bg__orb dynamic-bg__orb--c"></div>
            <div class="dynamic-bg__vignette"></div>
        </div>
    }
}
