use leptos::prelude::*;

use crate::app::{ThemeMode, use_site_preference};

#[component]
pub fn ThemeControls() -> impl IntoView {
    let preference = use_site_preference();

    view! {
        <div class="theme-controls flex shrink-0 items-center gap-2">
            <button
                type="button"
                class="control-btn"
                on:click=move |_| {
                    let next = preference.theme.get_untracked().toggle();
                    preference.set_theme.set(next);
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
