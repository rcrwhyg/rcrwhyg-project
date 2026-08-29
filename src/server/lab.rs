use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// One card in `/lab` (experiment / small demo).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabEntry {
    pub id: String,
    pub name: String,
    pub status: String,
    pub stack: String,
    pub blurb: String,
    #[serde(default)]
    pub link: Option<String>,
}

/// All lab entries (parse-order preserved).
#[server(ListLab)]
pub async fn list_lab() -> Result<Vec<LabEntry>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let Some(path) = super::articles::locate_data_file("data/lab.json") else {
            return Ok(Vec::new());
        };
        let raw = match std::fs::read_to_string(&path) {
            Ok(s) => s,
            Err(err) => {
                leptos::logging::log!("read lab.json error: {err}");
                return Ok(Vec::new());
            }
        };
        match serde_json::from_str::<Vec<LabEntry>>(&raw) {
            Ok(items) => Ok(items),
            Err(err) => {
                leptos::logging::log!("parse lab.json error: {err}");
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
    fn parse_lab_tolerates_missing_link() {
        let raw = r#"[
            { "id": "a", "name": "A", "status": "shipped",
              "stack": "Zig", "blurb": "tiny demo" }
        ]"#;
        let items: Vec<LabEntry> = serde_json::from_str(raw).expect("parse");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id, "a");
        assert!(items[0].link.is_none());
    }
}
