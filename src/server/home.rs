use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// One entry in the home page "recent" feed (articles only).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecentItem {
    pub title: String,
    pub href: String,
    #[serde(default)]
    pub date: Option<String>,
    #[serde(default)]
    pub summary: Option<String>,
}

/// Latest published articles for the home hub.
#[server(RecentItems)]
pub async fn recent_items() -> Result<Vec<RecentItem>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let items = super::articles::sorted_metas()
            .into_iter()
            .take(8)
            .map(|a| RecentItem {
                title: a.title,
                href: format!("/articles/{}", a.slug),
                date: a.date,
                summary: if a.summary.is_empty() {
                    None
                } else {
                    Some(a.summary)
                },
            })
            .collect();
        Ok(items)
    }
    #[cfg(not(feature = "ssr"))]
    {
        Ok(Vec::new())
    }
}
