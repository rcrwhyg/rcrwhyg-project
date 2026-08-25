use serde::{Deserialize, Serialize};

/// Site-owned blog/essay record (source of truth).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Post {
    pub id: Option<i64>,
    pub slug: String,
    pub title: String,
    pub summary: Option<String>,
    pub body_markdown: String,
    pub tags: Vec<String>,
    /// RFC3339 or human-readable date string until chrono wiring lands.
    pub published_at: Option<String>,
}

/// List-card projection (no body).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostSummary {
    pub id: Option<i64>,
    pub slug: String,
    pub title: String,
    pub summary: Option<String>,
    pub tags: Vec<String>,
    pub published_at: Option<String>,
}

impl From<&Post> for PostSummary {
    fn from(post: &Post) -> Self {
        Self {
            id: post.id,
            slug: post.slug.clone(),
            title: post.title.clone(),
            summary: post.summary.clone(),
            tags: post.tags.clone(),
            published_at: post.published_at.clone(),
        }
    }
}

/// Detail payload for the blog page (HTML rendered on the server).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostDetail {
    pub post: Post,
    pub body_html: String,
}

/// Admin create/update form payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostInput {
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub body_markdown: String,
    /// Comma-separated tag names.
    pub tags: String,
    pub publish: bool,
}
