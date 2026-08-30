use std::fmt::{Display, Formatter};
use std::str::FromStr;

use leptos::prelude::*;
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

/// hydrate 模式：从 local storage 读用户上次选的主题。SSR 用默认值。
#[cfg(feature = "hydrate")]
fn read_initial_from_storage() -> ThemeMode {
    let Some(window) = web_sys::window() else {
        return ThemeMode::default();
    };
    let Ok(Some(storage)) = window.local_storage() else {
        return ThemeMode::default();
    };
    let Ok(Some(s)) = storage.get_item("rcrwhyg.theme") else {
        return ThemeMode::default();
    };
    s.parse::<ThemeMode>().unwrap_or_default()
}

#[cfg(not(feature = "hydrate"))]
fn read_initial_from_storage() -> ThemeMode {
    ThemeMode::default()
}

/// hydrate 模式：写 local storage + 把 data-theme 同步到 <html>/<body>。
/// SSR 是 no-op。
#[cfg(feature = "hydrate")]
fn persist_and_apply(theme: ThemeMode) {
    let Some(window) = web_sys::window() else {
        return;
    };
    if let Ok(Some(storage)) = window.local_storage() {
        let _ = storage.set_item("rcrwhyg.theme", theme.as_str());
    }
    if let Some(document) = window.document() {
        if let Some(html) = document.document_element() {
            let _ = html.set_attribute("data-theme", theme.as_str());
        }
        if let Some(body) = document.body() {
            let _ = body.set_attribute("data-theme", theme.as_str());
        }
    }
}

#[cfg(not(feature = "hydrate"))]
fn persist_and_apply(_theme: ThemeMode) {}

/// 主题上下文。注：之前用 leptos-use 的 use_local_storage，跨 tab
/// 同步没问题但同 tab 内的 Signal 同步不稳——Effect 不会重跑、html
/// data-theme 不会变。换成手写的 RwSignal + 手动持久化 + html attr，
/// 一次走通、不再依赖第三方 hook 的内部行为。
pub fn provide_site_preference() {
    let initial = read_initial_from_storage();
    let theme = RwSignal::new(initial);
    let thaw_theme = RwSignal::new(initial.to_thaw());

    Effect::new(move |_| {
        let theme_value = theme.get();
        thaw_theme.set(theme_value.to_thaw());
        persist_and_apply(theme_value);
    });

    provide_context(SitePreference {
        theme: theme.into(),
        set_theme: theme.write_only(),
        thaw_theme,
    });
}

pub fn use_site_preference() -> SitePreference {
    expect_context::<SitePreference>()
}
