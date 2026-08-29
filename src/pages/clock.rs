use leptos::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;

/// Pomodoro timer. Pure-frontend — no server fn, no JS file.
///
/// Default work = 25min, break = 5min. The sky progress ring empties
/// clockwise; mint buttons (start/pause/reset) are the primary actions.
///
/// The tick loop is hydrate-only (needs `web_sys` + `wasm_bindgen`); on SSR
/// the page renders the static shell with a "client only" note.
#[component]
pub fn ClockPage() -> impl IntoView {
    view! {
        <section class="page-panel mx-auto my-8 max-w-md px-4">
            <h1 class="page-title mb-2 text-center">
                <span class="accent">"番茄钟"</span>
            </h1>
            <p class="mb-6 text-center dim-text">
                "纯前端专注计时。切到前台会更准。"
            </p>
            <ClockIsland />
        </section>
    }
}

/// Client-only timer. On SSR this still renders, but with a static
/// `25:00` and inert controls.
#[component]
fn ClockIsland() -> impl IntoView {
    let mode = RwSignal::new(0_u8);
    let total_secs: Signal<u64> = Signal::derive(move || match mode.get() {
        1 => 5 * 60,
        _ => 25 * 60,
    });
    let remaining_secs = RwSignal::new(25_u64 * 60);
    let running = RwSignal::new(false);

    // tick_handle / interval logic only exists on the client.
    #[cfg(feature = "hydrate")]
    let tick_handle: RwSignal<Option<i32>> = RwSignal::new(None);

    let switch_mode = move |next: u8| {
        #[cfg(feature = "hydrate")]
        {
            if let Some(h) = tick_handle.get_untracked() {
                if let Some(w) = web_sys::window() {
                    w.clear_interval_with_handle(h);
                }
                tick_handle.set(None);
            }
        }
        running.set(false);
        mode.set(next);
        remaining_secs.set(if next == 1 { 5 * 60 } else { 25 * 60 });
    };

    let start = move |_: leptos::ev::MouseEvent| {
        #[cfg(feature = "hydrate")]
        {
            if running.get_untracked() {
                return;
            }
            running.set(true);

            let cb = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                let next = remaining_secs.get_untracked().saturating_sub(1);
                remaining_secs.set(next);
                if next == 0 {
                    if let Some(h) = tick_handle.get_untracked() {
                        if let Some(w) = web_sys::window() {
                            w.clear_interval_with_handle(h);
                        }
                    }
                    tick_handle.set(None);
                    running.set(false);
                }
            }) as Box<dyn FnMut()>);

            if let Some(window) = web_sys::window() {
                if let Ok(handle) = window.set_interval_with_callback_and_timeout_and_arguments_0(
                    cb.as_ref().unchecked_ref(),
                    1000,
                ) {
                    tick_handle.set(Some(handle));
                } else {
                    running.set(false);
                }
            } else {
                running.set(false);
            }
            cb.forget();
        }
        // SSR: no-op — start button just toggles `running` but nothing
        // happens. The page is only useful after hydration anyway.
        #[cfg(not(feature = "hydrate"))]
        {
            let _ = running.get();
        }
    };

    let pause = move |_: leptos::ev::MouseEvent| {
        #[cfg(feature = "hydrate")]
        {
            if let Some(h) = tick_handle.get_untracked() {
                if let Some(w) = web_sys::window() {
                    w.clear_interval_with_handle(h);
                }
            }
            tick_handle.set(None);
        }
        running.set(false);
    };

    let reset = move |_: leptos::ev::MouseEvent| {
        // The pause callback expects a MouseEvent; pass a synthetic one.
        pause(leptos::ev::MouseEvent::new("click").unwrap());
        remaining_secs.set(total_secs.get_untracked());
    };

    // On client, clear any pending interval when the component unmounts.
    #[cfg(feature = "hydrate")]
    {
        let tick_handle_for_cleanup = tick_handle;
        Effect::new(move |_| {
            on_cleanup(move || {
                if let Some(h) = tick_handle_for_cleanup.get_untracked() {
                    if let Some(w) = web_sys::window() {
                        w.clear_interval_with_handle(h);
                    }
                }
            });
        });
    }

    // SVG ring (sky color, empties as time passes).
    let radius = 110.0_f64;
    let circumference = 2.0 * std::f64::consts::PI * radius;
    let dash_style = move || {
        let total = total_secs.get() as f64;
        let remaining = remaining_secs.get() as f64;
        let frac = if total > 0.0 {
            1.0 - remaining / total
        } else {
            0.0
        };
        let offset = circumference * frac.clamp(0.0, 1.0);
        format!(
            "stroke-dasharray: {:.2}; stroke-dashoffset: {:.2};",
            circumference, offset
        )
    };

    let time_text = Signal::derive(move || {
        let total = remaining_secs.get();
        let m = total / 60;
        let s = total % 60;
        format!("{:02}:{:02}", m, s)
    });

    let is_work = move || mode.get() == 0;
    let is_break = move || mode.get() == 1;

    view! {
        <div class="clock-wrap">
            <div class="clock-mode">
                <button
                    type="button"
                    class=move || if is_work() { "is-active" } else { "" }
                    on:click=move |_| switch_mode(0)
                >
                    "工作 25"
                </button>
                <button
                    type="button"
                    class=move || if is_break() { "is-active" } else { "" }
                    on:click=move |_| switch_mode(1)
                >
                    "休息 5"
                </button>
            </div>

            <div class="clock-ring">
                <svg viewBox="0 0 240 240" class="h-full w-full -rotate-90">
                    <circle
                        cx="120"
                        cy="120"
                        r=radius
                        fill="none"
                        stroke="var(--glass-border)"
                        stroke-width="10"
                    />
                    <circle
                        cx="120"
                        cy="120"
                        r=radius
                        fill="none"
                        stroke="var(--accent-2)"
                        stroke-width="10"
                        stroke-linecap="round"
                        attr:style=dash_style
                    />
                </svg>
                <div class="clock-time">{time_text}</div>
            </div>

            <div class="flex gap-3">
                <button
                    type="button"
                    class="btn"
                    on:click=start
                    disabled=move || running.get()
                >
                    {move || if running.get() { "进行中" } else { "开始" }}
                </button>
                <button
                    type="button"
                    class="btn secondary"
                    on:click=pause
                    disabled=move || !running.get()
                >
                    "暂停"
                </button>
                <button
                    type="button"
                    class="btn ghost"
                    on:click=reset
                >
                    "重置"
                </button>
            </div>
        </div>
    }
}
