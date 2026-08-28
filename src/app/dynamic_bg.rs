use leptos::prelude::*;

/// 冷静科技下的动态背景：网格/光斑 + 鼠标跟随高光。
/// 颜色与浓度全部由 style/tokens.css 的 token 控制（深/浅主题分别取值）。
#[component]
pub fn DynamicBackground() -> impl IntoView {
    let mx = RwSignal::new(50.0_f64);
    let my = RwSignal::new(40.0_f64);

    #[cfg(feature = "hydrate")]
    {
        Effect::new(move |_| {
            let listener = window_event_listener(leptos::ev::mousemove, move |ev| {
                let (w, h) = web_sys::window()
                    .map(|win| {
                        let w = win
                            .inner_width()
                            .ok()
                            .and_then(|v| v.as_f64())
                            .unwrap_or(1.0)
                            .max(1.0);
                        let h = win
                            .inner_height()
                            .ok()
                            .and_then(|v| v.as_f64())
                            .unwrap_or(1.0)
                            .max(1.0);
                        (w, h)
                    })
                    .unwrap_or((1.0, 1.0));

                mx.set(((ev.client_x() as f64) / w * 100.0).clamp(0.0, 100.0));
                my.set(((ev.client_y() as f64) / h * 100.0).clamp(0.0, 100.0));
            });
            on_cleanup(move || drop(listener));
        });
    }

    view! {
        <div class="cyber-bg" aria-hidden="true">
            <div class="cyber-bg__base"></div>
            <div class="cyber-bg__grid"></div>
            <div class="cyber-bg__beam cyber-bg__beam--a"></div>
            <div class="cyber-bg__beam cyber-bg__beam--b"></div>
            <div class="cyber-bg__orb cyber-bg__orb--a"></div>
            <div class="cyber-bg__orb cyber-bg__orb--b"></div>
            <div class="cyber-bg__orb cyber-bg__orb--c"></div>
            <div
                class="cyber-bg__cursor"
                style=move || {
                    format!(
                        "--mx: {:.2}%; --my: {:.2}%;",
                        mx.get(),
                        my.get()
                    )
                }
            ></div>
            <div class="cyber-bg__scan"></div>
            <div class="cyber-bg__vignette"></div>
        </div>
    }
}
