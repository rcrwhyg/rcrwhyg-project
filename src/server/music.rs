use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// One track in `/music`. Exactly one of `src` (self-hosted media path)
/// or `embed_url` (third-party iframe) must be set; both null is allowed as
/// a placeholder entry (e.g. TBD).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MusicEntry {
    pub id: String,
    pub title: String,
    pub artist: String,
    #[serde(default)]
    pub src: Option<String>,
    #[serde(default)]
    pub embed_url: Option<String>,
    #[serde(default)]
    pub note: Option<String>,
}

/// All music entries (parse-order preserved).
#[server(ListMusic)]
pub async fn list_music() -> Result<Vec<MusicEntry>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let Some(path) = super::articles::locate_data_file("data/music.json") else {
            return Ok(Vec::new());
        };
        let raw = match std::fs::read_to_string(&path) {
            Ok(s) => s,
            Err(err) => {
                leptos::logging::log!("read music.json error: {err}");
                return Ok(Vec::new());
            }
        };
        match serde_json::from_str::<Vec<MusicEntry>>(&raw) {
            Ok(items) => Ok(items),
            Err(err) => {
                leptos::logging::log!("parse music.json error: {err}");
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
    fn parse_music_allows_placeholder() {
        let raw = r#"[
            { "id": "track-1", "title": "x", "artist": "y",
              "src": null, "embed_url": null, "note": "TBD" }
        ]"#;
        let items: Vec<MusicEntry> = serde_json::from_str(raw).expect("parse");
        assert_eq!(items.len(), 1);
        assert!(items[0].src.is_none());
        assert!(items[0].embed_url.is_none());
    }

    #[test]
    fn parse_music_accepts_src() {
        let raw = r#"[
            { "id": "a", "title": "t", "artist": "ar", "src": "/media/a.mp3" }
        ]"#;
        let items: Vec<MusicEntry> = serde_json::from_str(raw).expect("parse");
        assert_eq!(items[0].src.as_deref(), Some("/media/a.mp3"));
    }
}
