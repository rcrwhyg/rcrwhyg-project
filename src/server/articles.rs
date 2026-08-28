use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use std::path::PathBuf;

/// Article metadata parsed from a `articles/*.md` file.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArticleMeta {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub date: Option<String>,
}

/// Detail payload for `/articles/:slug` (body rendered server-side).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArticleRendered {
    pub meta: ArticleMeta,
    pub body_html: String,
}

/// List of all articles (newest first).
#[server(ListSiteArticles)]
pub async fn list_site_articles() -> Result<Vec<ArticleMeta>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        Ok(sorted_metas())
    }
    #[cfg(not(feature = "ssr"))]
    {
        Ok(Vec::new())
    }
}

/// Single article by slug, body rendered to HTML.
#[server(GetSiteArticle)]
pub async fn get_site_article(slug: String) -> Result<Option<ArticleRendered>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let slug = slug.trim().to_string();
        if slug.is_empty() {
            return Ok(None);
        }
        let path = locate_articles_dir().map(|d| d.join(format!("{slug}.md")));
        let Some(path) = path else { return Ok(None) };
        if !path.is_file() {
            return Ok(None);
        }
        let markdown = match std::fs::read_to_string(&path) {
            Ok(s) => s,
            Err(err) => {
                leptos::logging::log!("read article {slug} error: {err}");
                return Ok(None);
            }
        };
        let meta = parse_meta(&markdown, &slug);
        let body = strip_boilerplate(&markdown);
        Ok(Some(ArticleRendered {
            meta,
            body_html: super::posts::markdown_to_html(&body),
        }))
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = slug;
        Ok(None)
    }
}

#[cfg(feature = "ssr")]
fn locate_articles_dir() -> Option<PathBuf> {
    let cwd = std::env::current_dir().ok()?;
    let root = std::env::var("LEPTOS_SITE_ROOT").unwrap_or_else(|_| "target/site".to_string());
    [cwd.join(&root).join("articles"), cwd.join("articles")]
        .into_iter()
        .find(|p| p.is_dir())
}

#[cfg(feature = "ssr")]
fn sorted_metas() -> Vec<ArticleMeta> {
    let Some(dir) = locate_articles_dir() else {
        return Vec::new();
    };
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return Vec::new();
    };
    let mut metas = entries
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().map(|x| x == "md").unwrap_or(false))
        .filter_map(|e| {
            let slug = e
                .file_name()
                .to_string_lossy()
                .trim_end_matches(".md")
                .to_string();
            let body = std::fs::read_to_string(e.path()).ok()?;
            Some(parse_meta(&body, &slug))
        })
        .collect::<Vec<_>>();
    metas.sort_by(|a, b| b.date.cmp(&a.date));
    metas
}

#[cfg(feature = "ssr")]
fn parse_meta(markdown: &str, slug: &str) -> ArticleMeta {
    let mut title = String::new();
    let mut summary = String::new();
    let mut date = None;
    for line in markdown.lines() {
        let t = line.trim_start();
        if title.is_empty() && t.starts_with("# ") {
            title = t.trim_start_matches("# ").trim().to_string();
        } else if line.starts_with("> **摘要**:") {
            summary = line.trim_start_matches("> **摘要**:").trim().to_string();
        } else if date.is_none()
            && let Some(pos) = line.find("**版本信息**")
        {
            date = extract_date(&line[pos..]);
        }
        if !title.is_empty() && !summary.is_empty() && date.is_some() {
            break;
        }
    }
    ArticleMeta {
        slug: slug.to_string(),
        title,
        summary,
        date,
    }
}

/// Pull a `YYYY-MM`-style date out of `写于 2026-08。`.
#[cfg(feature = "ssr")]
fn extract_date(s: &str) -> Option<String> {
    let idx = s.find("写于 ")? + "写于 ".len();
    let rest: String = s[idx..]
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '-')
        .collect();
    (!rest.is_empty()).then_some(rest)
}

/// Drop the `# 标题` line and the `> **摘要**:` line; page chrome shows them.
#[cfg(feature = "ssr")]
fn strip_boilerplate(markdown: &str) -> String {
    markdown
        .lines()
        .filter(|l| !l.trim_start().starts_with("# ") && !l.starts_with("> **摘要**:"))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(all(test, feature = "ssr"))]
mod tests {
    use super::*;

    const SAMPLE: &str = concat!(
        "# 测试标题\n",
        "\n",
        "> **摘要**: 这是一段摘要内容，用于测试。\n",
        "\n",
        "## 正文\n",
        "\n",
        "内容段落。\n",
        "\n",
        "## 总结\n",
        "\n",
        "**版本信息**: 本文基于某版本，写于 2026-08。\n",
    );

    #[test]
    fn parse_meta_extracts_title_summary_date() {
        let meta = parse_meta(SAMPLE, "sample");
        assert_eq!(meta.slug, "sample");
        assert_eq!(meta.title, "测试标题");
        assert_eq!(meta.summary, "这是一段摘要内容，用于测试。");
        assert_eq!(meta.date.as_deref(), Some("2026-08"));
    }

    #[test]
    fn strip_boilerplate_removes_title_and_summary() {
        let body = strip_boilerplate(SAMPLE);
        assert!(!body.contains("测试标题"));
        assert!(!body.contains("**摘要**"));
        assert!(body.contains("## 正文"));
        assert!(body.contains("**版本信息**"));
        let html = super::super::posts::markdown_to_html(&body);
        assert!(html.contains("<h2>正文</h2>"));
    }

    #[test]
    fn extract_date_tolerates_missing() {
        assert_eq!(extract_date("**版本信息**: 无日期。"), None);
        assert_eq!(
            extract_date("**版本信息**: 写于 2026-08。"),
            Some("2026-08".into())
        );
    }
}
