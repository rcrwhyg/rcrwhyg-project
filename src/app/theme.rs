use std::fmt::{Display, Formatter};
use std::str::FromStr;

use codee::string::FromToStringCodec;
use leptos::prelude::*;
use leptos_use::storage::use_local_storage;
use thaw::Theme;

/// Only dark / light — no other style modes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ThemeMode {
    #[default]
    Dark,
    Light,
}

impl ThemeMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Dark => "dark",
            Self::Light => "light",
        }
    }

    pub fn toggle(self) -> Self {
        match self {
            Self::Dark => Self::Light,
            Self::Light => Self::Dark,
        }
    }

    pub fn to_thaw(self) -> Theme {
        match self {
            Self::Dark => Theme::dark(),
            Self::Light => Theme::light(),
        }
    }
}

impl Display for ThemeMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for ThemeMode {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(match value {
            "light" => Self::Light,
            _ => Self::Dark,
        })
    }
}

#[derive(Clone, Copy)]
pub struct SitePreference {
    pub theme: Signal<ThemeMode>,
    pub set_theme: WriteSignal<ThemeMode>,
    pub thaw_theme: RwSignal<Theme>,
}

pub fn provide_site_preference() {
    let (theme, set_theme, _) = use_local_storage::<ThemeMode, FromToStringCodec>("rcrwhyg.theme");

    let thaw_theme = RwSignal::new(theme.get_untracked().to_thaw());

    Effect::new(move |_| {
        let theme_value = theme.get();
        thaw_theme.set(theme_value.to_thaw());

        #[cfg(feature = "hydrate")]
        {
            if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                if let Some(html) = document.document_element() {
                    let _ = html.set_attribute("data-theme", theme_value.as_str());
                }
                if let Some(body) = document.body() {
                    let _ = body.set_attribute("data-theme", theme_value.as_str());
                }
            }
        }
    });

    provide_context(SitePreference {
        theme,
        set_theme,
        thaw_theme,
    });
}

pub fn use_site_preference() -> SitePreference {
    expect_context::<SitePreference>()
}
