use leptos::prelude::*;
use thaw::Theme;

/// Fixed dark theme — thaw UI provider only (no user toggle).
#[derive(Clone, Copy)]
pub struct SitePreference {
    pub thaw_theme: RwSignal<Theme>,
}

pub fn provide_site_preference() {
    provide_context(SitePreference {
        thaw_theme: RwSignal::new(Theme::dark()),
    });
}

pub fn use_site_preference() -> SitePreference {
    expect_context::<SitePreference>()
}
