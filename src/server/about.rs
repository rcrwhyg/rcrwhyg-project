use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// About page payload — `body_html` is the rendered markdown.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AboutContent {
    pub body_html: String,
}

/// Read `content/about.md`, render to HTML.
#[server(GetAbout)]
pub async fn get_about() -> Result<Option<AboutContent>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let Some(path) = super::articles::locate_data_file("content/about.md") else {
            return Ok(None);
        };
        let raw = match std::fs::read_to_string(&path) {
            Ok(s) => s,
            Err(err) => {
                leptos::logging::log!("read content/about.md error: {err}");
                return Ok(None);
            }
        };
        let body_html = super::markdown::markdown_to_html(&raw);
        if body_html.trim().is_empty() {
            return Ok(None);
        }
        Ok(Some(AboutContent { body_html }))
    }
    #[cfg(not(feature = "ssr"))]
    {
        Ok(None)
    }
}

#[cfg(all(test, feature = "ssr"))]
mod tests {
    #[test]
    fn markdown_to_html_handles_about_markdown() {
        // Same renderer used by `get_about` and `get_site_article`.
        let md = "## 创刊叙事\n\n一个站点。\n\n## 联系方式\n\n- 邮件\n";
        let html = crate::server::markdown::markdown_to_html(md);
        assert!(html.contains("<h2>创刊叙事</h2>"));
        assert!(html.contains("<ul>"));
    }
}
