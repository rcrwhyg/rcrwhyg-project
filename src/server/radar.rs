use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// One row in `/radar` (multi-ecosystem learning progress).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RadarEntry {
    pub ecosystem: String,
    pub status: String,
    pub accent: String,
    #[serde(default)]
    pub note: Option<String>,
    #[serde(default)]
    pub link: Option<String>,
}

/// All radar entries (parse-order preserved; sort is the caller's choice).
#[server(ListRadar)]
pub async fn list_radar() -> Result<Vec<RadarEntry>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let Some(path) = super::articles::locate_data_file("data/radar.json") else {
            return Ok(Vec::new());
        };
        let raw = match std::fs::read_to_string(&path) {
            Ok(s) => s,
            Err(err) => {
                leptos::logging::log!("read radar.json error: {err}");
                return Ok(Vec::new());
            }
        };
        match serde_json::from_str::<Vec<RadarEntry>>(&raw) {
            Ok(items) => Ok(items),
            Err(err) => {
                leptos::logging::log!("parse radar.json error: {err}");
                Ok(Vec::new())
            }
        }
    }
    #[cfg(not(feature = "ssr"))]
    {
        Ok(Vec::new())
    }
}

#[cfg(all(test, feature = "ssr"))]
mod tests {
    use super::*;

    #[test]
    fn parse_radar_handles_minimal_schema() {
        let raw = r#"[
            { "ecosystem": "Rust", "status": "主力", "accent": "mint" },
            { "ecosystem": "React", "status": "辅助", "accent": "sky",
              "note": "了解 hooks 模式", "link": "https://react.dev" }
        ]"#;
        let items: Vec<RadarEntry> = serde_json::from_str(raw).expect("parse");
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].ecosystem, "Rust");
        assert_eq!(items[0].accent, "mint");
        assert!(items[0].note.is_none());
        assert_eq!(items[1].link.as_deref(), Some("https://react.dev"));
    }
}
