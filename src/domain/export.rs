use crate::domain::Post;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MarkdownBundle {
    pub frontmatter_title: String,
    pub frontmatter_slug: String,
    pub frontmatter_tags: Vec<String>,
    pub frontmatter_summary: Option<String>,
    pub frontmatter_published_at: Option<String>,
    pub body_markdown: String,
}

impl MarkdownBundle {
    pub fn to_markdown(&self) -> String {
        let tags = self.frontmatter_tags.join(", ");
        let summary = self.frontmatter_summary.clone().unwrap_or_default();
        let published = self
            .frontmatter_published_at
            .clone()
            .unwrap_or_else(|| String::from(""));
        format!(
            "---\ntitle: {}\nslug: {}\ntags: [{}]\nsummary: {}\npublished_at: {}\n---\n\n{}",
            self.frontmatter_title,
            self.frontmatter_slug,
            tags,
            summary,
            published,
            self.body_markdown
        )
    }

    pub fn from_post(post: &Post) -> Self {
        Self {
            frontmatter_title: post.title.clone(),
            frontmatter_slug: post.slug.clone(),
            frontmatter_tags: post.tags.clone(),
            frontmatter_summary: post.summary.clone(),
            frontmatter_published_at: post.published_at.clone(),
            body_markdown: post.body_markdown.clone(),
        }
    }

    pub fn into_post(self, id: Option<i64>) -> Post {
        Post {
            id,
            slug: self.frontmatter_slug,
            title: self.frontmatter_title,
            summary: self.frontmatter_summary,
            body_markdown: self.body_markdown,
            tags: self.frontmatter_tags,
            published_at: self.frontmatter_published_at,
        }
    }
}

#[derive(Debug)]
pub enum ExportError {
    EmptyBody,
}

impl std::fmt::Display for ExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyBody => write!(f, "post body is empty"),
        }
    }
}

impl std::error::Error for ExportError {}

#[derive(Debug)]
pub enum ImportError {
    MissingFrontmatter,
    MissingTitle,
    MissingSlug,
}

impl std::fmt::Display for ImportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingFrontmatter => write!(f, "missing yaml frontmatter"),
            Self::MissingTitle => write!(f, "missing title in frontmatter"),
            Self::MissingSlug => write!(f, "missing slug in frontmatter"),
        }
    }
}

impl std::error::Error for ImportError {}

pub trait ContentExporter {
    fn export_post(&self, post: &Post) -> Result<MarkdownBundle, ExportError>;
}

pub trait ContentImporter {
    fn import_markdown(&self, raw: &str) -> Result<MarkdownBundle, ImportError>;
}

#[derive(Default)]
pub struct MarkdownExporter;

impl ContentExporter for MarkdownExporter {
    fn export_post(&self, post: &Post) -> Result<MarkdownBundle, ExportError> {
        if post.body_markdown.trim().is_empty() {
            return Err(ExportError::EmptyBody);
        }
        Ok(MarkdownBundle::from_post(post))
    }
}

#[derive(Default)]
pub struct MarkdownImporter;

impl ContentImporter for MarkdownImporter {
    fn import_markdown(&self, raw: &str) -> Result<MarkdownBundle, ImportError> {
        let raw = raw.trim_start();
        if !raw.starts_with("---") {
            return Err(ImportError::MissingFrontmatter);
        }
        let rest = &raw[3..];
        let end = rest
            .find("\n---")
            .ok_or(ImportError::MissingFrontmatter)?;
        let fm = &rest[..end];
        let body = rest[end + 4..].trim_start_matches('\n').to_string();

        let mut title = None;
        let mut slug = None;
        let mut summary = None;
        let mut published_at = None;
        let mut tags = Vec::new();

        for line in fm.lines() {
            let line = line.trim();
            if let Some(value) = line.strip_prefix("title:") {
                title = Some(value.trim().to_string());
            } else if let Some(value) = line.strip_prefix("slug:") {
                slug = Some(value.trim().to_string());
            } else if let Some(value) = line.strip_prefix("summary:") {
                let v = value.trim();
                if !v.is_empty() {
                    summary = Some(v.to_string());
                }
            } else if let Some(value) = line.strip_prefix("published_at:") {
                let v = value.trim();
                if !v.is_empty() {
                    published_at = Some(v.to_string());
                }
            } else if let Some(value) = line.strip_prefix("tags:") {
                let v = value.trim().trim_start_matches('[').trim_end_matches(']');
                tags = v
                    .split(',')
                    .map(|t| t.trim().to_string())
                    .filter(|t| !t.is_empty())
                    .collect();
            }
        }

        Ok(MarkdownBundle {
            frontmatter_title: title.ok_or(ImportError::MissingTitle)?,
            frontmatter_slug: slug.ok_or(ImportError::MissingSlug)?,
            frontmatter_tags: tags,
            frontmatter_summary: summary,
            frontmatter_published_at: published_at,
            body_markdown: body,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_markdown_bundle() {
        let post = Post {
            id: Some(1),
            slug: "hello".into(),
            title: "Hello".into(),
            summary: Some("sum".into()),
            body_markdown: "body text".into(),
            tags: vec!["rust".into()],
            published_at: Some("2026-07-23".into()),
        };
        let exported = MarkdownExporter
            .export_post(&post)
            .expect("export")
            .to_markdown();
        let imported = MarkdownImporter
            .import_markdown(&exported)
            .expect("import");
        assert_eq!(imported.frontmatter_slug, "hello");
        assert_eq!(imported.body_markdown, "body text");
    }
}
