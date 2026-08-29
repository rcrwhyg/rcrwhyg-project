use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// One entry in the home page "recent" feed.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecentItem {
    pub kind: String, // "article" | "tool"
    pub title: String,
    pub href: String,
    #[serde(default)]
    pub date: Option<String>,
    #[serde(default)]
    pub summary: Option<String>,
}

/// Recent updates for the home hub: latest articles first, then registry tools.
#[server(RecentItems)]
pub async fn recent_items() -> Result<Vec<RecentItem>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let mut items: Vec<RecentItem> = Vec::new();

        for a in super::articles::sorted_metas() {
            items.push(RecentItem {
                kind: "article".into(),
                title: a.title,
                href: format!("/articles/{}", a.slug),
                date: a.date,
                summary: if a.summary.is_empty() {
                    None
                } else {
                    Some(a.summary)
                },
            });
        }

        for tool in crate::tools::registry::all_tools() {
            items.push(RecentItem {
                kind: "tool".into(),
                title: tool.title.to_string(),
                href: tool.path.to_string(),
                date: None,
                summary: Some(tool.summary.to_string()),
            });
        }

        // Articles carry a `YYYY-MM` date; tools have none and fall to the
        // bottom. The home view caps this further.
        items.sort_by(|a, b| match (a.date.as_deref(), b.date.as_deref()) {
            (Some(x), Some(y)) => y.cmp(x),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        });
        items.truncate(8);
        Ok(items)
    }
    #[cfg(not(feature = "ssr"))]
    {
        Ok(Vec::new())
    }
}
