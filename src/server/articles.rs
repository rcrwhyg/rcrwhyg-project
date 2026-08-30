use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use std::path::{Path, PathBuf};

/// Article metadata parsed from `articles/**/*.md`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArticleMeta {
    /// URL slug（文件名，不含路径），如 `01-the-modest-start`。
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub date: Option<String>,
    /// 合集目录 slug，如 `2c2g-server`；根目录单篇为 `None`。
    pub collection_slug: Option<String>,
    /// 合集展示名（来自子目录 `_meta.json`）。
    pub collection_title: Option<String>,
    /// 文件名前缀编号（`06-foo` → 6），用于倒序。
    pub file_order: u32,
}

/// 尚未有文章的合集占位（来自 `_meta.json` 的 `placeholder: true`）。
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionPlaceholder {
    pub slug: String,
    pub title: String,
    pub note: Option<String>,
    /// 来自 `_meta.json` 的 `order`，用于占位合集排序。
    pub order: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArticleListView {
    pub articles: Vec<ArticleMeta>,
    pub placeholders: Vec<CollectionPlaceholder>,
}

/// Flat list (newest first) + collection placeholders.
#[server(ListSiteArticles)]
pub async fn list_site_articles() -> Result<ArticleListView, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        Ok(build_list_view())
    }
    #[cfg(not(feature = "ssr"))]
    {
        Ok(ArticleListView {
            articles: Vec::new(),
            placeholders: Vec::new(),
        })
    }
}

/// Detail payload for `/articles/:slug`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArticleRendered {
    pub meta: ArticleMeta,
    pub body_html: String,
}

#[server(GetSiteArticle)]
pub async fn get_site_article(slug: String) -> Result<Option<ArticleRendered>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let slug = slug.trim().to_string();
        if slug.is_empty() || !is_publishable_filename(&slug) {
            return Ok(None);
        }
        let Some(path) = find_article_path(&slug) else {
            return Ok(None);
        };
        let markdown = match std::fs::read_to_string(&path) {
            Ok(s) => s,
            Err(err) => {
                leptos::logging::log!("read article {slug} error: {err}");
                return Ok(None);
            }
        };
        if !site_publishes(&markdown) {
            return Ok(None);
        }
        let meta = parse_meta(&markdown, &slug, &path);
        let body = strip_boilerplate(&markdown);
        Ok(Some(ArticleRendered {
            meta,
            body_html: super::markdown::markdown_to_html(&body),
        }))
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = slug;
        Ok(None)
    }
}

#[cfg(feature = "ssr")]
#[derive(Clone, Debug)]
struct CollectionMeta {
    slug: String,
    title: String,
    order: u32,
    #[allow(dead_code)]
    placeholder: bool,
    note: Option<String>,
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
pub(crate) fn locate_data_file(rel: &str) -> Option<PathBuf> {
    let cwd = std::env::current_dir().ok()?;
    let root = std::env::var("LEPTOS_SITE_ROOT").unwrap_or_else(|_| "target/site".to_string());
    [cwd.join(&root).join(rel), cwd.join(rel)]
        .into_iter()
        .find(|p| p.is_file())
}

#[cfg(feature = "ssr")]
fn build_list_view() -> ArticleListView {
    let Some(root) = locate_articles_dir() else {
        return ArticleListView {
            articles: Vec::new(),
            placeholders: Vec::new(),
        };
    };
    let mut articles = Vec::new();
    let mut placeholders = Vec::new();
    collect_from_dir(&root, None, &mut articles, &mut placeholders);
    articles.sort_by(compare_newest_first);
    placeholders.sort_by(|a, b| a.order.cmp(&b.order).then_with(|| a.title.cmp(&b.title)));
    ArticleListView {
        articles,
        placeholders,
    }
}

#[cfg(feature = "ssr")]
pub(crate) fn sorted_metas() -> Vec<ArticleMeta> {
    build_list_view().articles
}

#[cfg(feature = "ssr")]
fn collect_from_dir(
    dir: &Path,
    collection: Option<CollectionMeta>,
    articles: &mut Vec<ArticleMeta>,
    placeholders: &mut Vec<CollectionPlaceholder>,
) {
    let entries: Vec<_> = std::fs::read_dir(dir)
        .ok()
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
        .collect();

    if let Some(ref col) = collection
        && col.placeholder
    {
        let has_article = entries
            .iter()
            .any(|e| e.path().extension().is_some_and(|x| x == "md"));
        if !has_article {
            placeholders.push(CollectionPlaceholder {
                slug: col.slug.clone(),
                title: col.title.clone(),
                note: col.note.clone(),
                order: col.order,
            });
        }
    }

    for entry in entries {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        if path.is_dir() {
            if matches!(name.as_str(), "templates" | "template") {
                continue;
            }
            let meta = load_collection_meta(&path, &name);
            collect_from_dir(&path, Some(meta), articles, placeholders);
            continue;
        }

        if path.extension().is_none_or(|x| x != "md") {
            continue;
        }
        if name.eq_ignore_ascii_case("README.md") {
            continue;
        }

        let slug = name.trim_end_matches(".md").to_string();
        if !is_publishable_filename(&slug) {
            continue;
        }
        let Ok(body) = std::fs::read_to_string(&path) else {
            continue;
        };
        if !site_publishes(&body) {
            continue;
        }
        articles.push(parse_meta(&body, &slug, &path));
    }
}

#[cfg(feature = "ssr")]
fn load_collection_meta(dir: &Path, slug: &str) -> CollectionMeta {
    let meta_path = dir.join("_meta.json");
    if let Ok(raw) = std::fs::read_to_string(&meta_path)
        && let Ok(v) = serde_json::from_str::<serde_json::Value>(&raw)
    {
        let title = v
            .get("title")
            .and_then(|t| t.as_str())
            .unwrap_or(slug)
            .to_string();
        let order = v.get("order").and_then(|o| o.as_u64()).unwrap_or(99) as u32;
        let placeholder = v
            .get("placeholder")
            .and_then(|p| p.as_bool())
            .unwrap_or(false);
        let note = v.get("note").and_then(|n| n.as_str()).map(str::to_string);
        return CollectionMeta {
            slug: slug.to_string(),
            title,
            order,
            placeholder,
            note,
        };
    }
    CollectionMeta {
        slug: slug.to_string(),
        title: slug.to_string(),
        order: 99,
        placeholder: false,
        note: None,
    }
}

#[cfg(feature = "ssr")]
fn find_article_path(slug: &str) -> Option<PathBuf> {
    let root = locate_articles_dir()?;
    find_in_dir(&root, slug)
}

#[cfg(feature = "ssr")]
fn find_in_dir(dir: &Path, slug: &str) -> Option<PathBuf> {
    let entries = std::fs::read_dir(dir).ok()?;
    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            if let Some(found) = find_in_dir(&path, slug) {
                return Some(found);
            }
            continue;
        }
        if path.extension().is_none_or(|x| x != "md") {
            continue;
        }
        let stem = path.file_stem()?.to_string_lossy();
        if stem == slug {
            return Some(path);
        }
    }
    None
}

#[cfg(feature = "ssr")]
fn compare_newest_first(a: &ArticleMeta, b: &ArticleMeta) -> std::cmp::Ordering {
    b.file_order
        .cmp(&a.file_order)
        .then_with(|| match (a.date.as_deref(), b.date.as_deref()) {
            (Some(x), Some(y)) => y.cmp(x),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        })
        .then_with(|| b.slug.cmp(&a.slug))
}

#[cfg(feature = "ssr")]
fn is_publishable_filename(stem: &str) -> bool {
    stem.len() >= 4
        && stem.as_bytes().get(2) == Some(&b'-')
        && stem[..2].chars().all(|c| c.is_ascii_digit())
}

#[cfg(feature = "ssr")]
fn file_order_from_slug(slug: &str) -> u32 {
    slug.get(..2).and_then(|n| n.parse().ok()).unwrap_or(0)
}

#[cfg(feature = "ssr")]
fn site_publishes(markdown: &str) -> bool {
    for line in markdown.lines() {
        let t = line.trim();
        if t.starts_with("> **站点发布**:") {
            let v = t.trim_start_matches("> **站点发布**:").trim();
            return !matches!(v, "否" | "false" | "no");
        }
    }
    true
}

#[cfg(feature = "ssr")]
fn parse_meta(markdown: &str, slug: &str, path: &Path) -> ArticleMeta {
    let mut title = String::new();
    let mut summary = String::new();
    let mut date = None;

    for line in markdown.lines() {
        let t = line.trim_start();
        if title.is_empty() && t.starts_with("# ") {
            title = t.trim_start_matches("# ").trim().to_string();
        } else if line.starts_with("> **摘要**:") {
            summary = line.trim_start_matches("> **摘要**:").trim().to_string();
        } else if date.is_none() && line.contains("**版本信息**") {
            date = extract_date(line);
        }
    }

    let (collection_slug, collection_title) = collection_for_path(path);

    ArticleMeta {
        slug: slug.to_string(),
        title,
        summary,
        date,
        collection_slug,
        collection_title,
        file_order: file_order_from_slug(slug),
    }
}

#[cfg(feature = "ssr")]
fn collection_for_path(path: &Path) -> (Option<String>, Option<String>) {
    let Some(root) = locate_articles_dir() else {
        return (None, None);
    };
    let Ok(rel) = path.strip_prefix(&root) else {
        return (None, None);
    };
    let Some(parent) = rel.parent() else {
        return (None, None);
    };
    if parent.as_os_str().is_empty() {
        return (None, None);
    }
    let slug = parent
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();
    if slug.is_empty() || slug == "templates" {
        return (None, None);
    }
    let meta = load_collection_meta(&root.join(&slug), &slug);
    (Some(slug), Some(meta.title))
}

#[cfg(feature = "ssr")]
fn extract_date(s: &str) -> Option<String> {
    let idx = s.find("写于 ")? + "写于 ".len();
    let rest: String = s[idx..]
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '-')
        .collect();
    (!rest.is_empty()).then_some(rest)
}

#[cfg(feature = "ssr")]
fn strip_boilerplate(markdown: &str) -> String {
    markdown
        .lines()
        .filter(|l| {
            let t = l.trim_start();
            !(t.starts_with("# ")
                || l.starts_with("> **摘要**:")
                || l.starts_with("> **合集**:")
                || l.starts_with("> **合集序**:")
                || l.starts_with("> **站点发布**:"))
        })
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
        "**版本信息**: 本文基于某版本，写于 2026-08。\n",
    );

    #[test]
    fn parse_meta_extracts_fields() {
        let path = PathBuf::from("articles/2c2g-server/02-sample.md");
        let meta = parse_meta(SAMPLE, "02-sample", &path);
        assert_eq!(meta.slug, "02-sample");
        assert_eq!(meta.title, "测试标题");
        assert_eq!(meta.file_order, 2);
        assert_eq!(meta.date.as_deref(), Some("2026-08"));
    }

    #[test]
    fn publishable_filename_pattern() {
        assert!(is_publishable_filename("01-the-modest-start"));
        assert!(!is_publishable_filename("README"));
    }

    #[test]
    fn sort_newest_file_prefix_first() {
        let a = ArticleMeta {
            slug: "01-a".into(),
            title: String::new(),
            summary: String::new(),
            date: None,
            collection_slug: None,
            collection_title: None,
            file_order: 1,
        };
        let b = ArticleMeta {
            slug: "06-b".into(),
            title: String::new(),
            summary: String::new(),
            date: None,
            collection_slug: None,
            collection_title: None,
            file_order: 6,
        };
        let mut v = [a, b];
        v.sort_by(compare_newest_first);
        assert_eq!(v[0].slug, "06-b");
    }

    #[test]
    fn site_publish_false_excluded() {
        let md = "# x\n\n> **站点发布**: 否\n";
        assert!(!site_publishes(md));
    }
}
