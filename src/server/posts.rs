use leptos::prelude::*;

use crate::domain::{Post, PostDetail, PostInput, PostSummary};
#[cfg(feature = "ssr")]
use crate::domain::seed_posts;

/// Published posts for the blog index (DB when available, else seed).
#[server(ListPublishedPosts)]
pub async fn list_published_posts() -> Result<Vec<PostSummary>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        match load_from_db().await {
            Ok(Some(posts)) if !posts.is_empty() => {
                return Ok(posts.iter().map(PostSummary::from).collect());
            }
            Ok(_) => {}
            Err(err) => {
                leptos::logging::log!("list_published_posts db error: {err}; falling back to seed");
            }
        }
        return Ok(seed_posts().iter().map(PostSummary::from).collect());
    }
    #[cfg(not(feature = "ssr"))]
    {
        Ok(Vec::new())
    }
}

/// Single post by slug, with server-rendered Markdown HTML.
#[server(GetPostBySlug)]
pub async fn get_post_by_slug(slug: String) -> Result<Option<PostDetail>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let slug = slug.trim().to_string();
        if slug.is_empty() {
            return Ok(None);
        }

        match load_one_from_db(&slug).await {
            Ok(Some(post)) => return Ok(Some(to_detail(post))),
            Ok(None) => {}
            Err(err) => {
                leptos::logging::log!("get_post_by_slug db error: {err}; falling back to seed");
            }
        }

        Ok(seed_posts()
            .into_iter()
            .find(|p| p.slug == slug)
            .map(to_detail))
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = slug;
        Ok(None)
    }
}

#[cfg(feature = "ssr")]
fn to_detail(post: Post) -> PostDetail {
    let body_html = markdown_to_html(&post.body_markdown);
    PostDetail { post, body_html }
}

#[cfg(feature = "ssr")]
pub(crate) fn markdown_to_html(markdown: &str) -> String {
    use pulldown_cmark::{Options, Parser, html};

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(markdown, options);
    let mut html_out = String::new();
    html::push_html(&mut html_out, parser);
    html_out
}

#[cfg(feature = "ssr")]
async fn load_from_db() -> Result<Option<Vec<Post>>, String> {
    let Some(pool) = use_context::<sqlx::PgPool>() else {
        return Ok(None);
    };
    Ok(Some(fetch_published_posts(&pool).await?))
}

#[cfg(feature = "ssr")]
async fn load_one_from_db(slug: &str) -> Result<Option<Post>, String> {
    let Some(pool) = use_context::<sqlx::PgPool>() else {
        return Ok(None);
    };
    fetch_post_by_slug(&pool, slug).await
}

/// DB access used by server fns and integration tests (shared pool).
#[cfg(feature = "ssr")]
pub(crate) async fn fetch_published_posts(pool: &sqlx::PgPool) -> Result<Vec<Post>, String> {
    #[derive(sqlx::FromRow)]
    struct Row {
        id: i64,
        slug: String,
        title: String,
        summary: Option<String>,
        body_markdown: String,
        published_at: Option<chrono::DateTime<chrono::Utc>>,
    }

    let rows = sqlx::query_as::<_, Row>(
        r#"
        SELECT id, slug, title, summary, body_markdown, published_at
        FROM posts
        WHERE published_at IS NOT NULL
        ORDER BY published_at DESC NULLS LAST, id DESC
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    let mut posts = Vec::with_capacity(rows.len());
    for row in rows {
        let tags = load_tags(pool, row.id).await.unwrap_or_default();
        posts.push(Post {
            id: Some(row.id),
            slug: row.slug,
            title: row.title,
            summary: row.summary,
            body_markdown: row.body_markdown,
            tags,
            published_at: row.published_at.map(|t| t.date_naive().to_string()),
        });
    }
    Ok(posts)
}

#[cfg(feature = "ssr")]
pub(crate) async fn fetch_post_by_slug(
    pool: &sqlx::PgPool,
    slug: &str,
) -> Result<Option<Post>, String> {
    #[derive(sqlx::FromRow)]
    struct Row {
        id: i64,
        slug: String,
        title: String,
        summary: Option<String>,
        body_markdown: String,
        published_at: Option<chrono::DateTime<chrono::Utc>>,
    }

    let row = sqlx::query_as::<_, Row>(
        r#"
        SELECT id, slug, title, summary, body_markdown, published_at
        FROM posts
        WHERE slug = $1 AND published_at IS NOT NULL
        LIMIT 1
        "#,
    )
    .bind(slug)
    .fetch_optional(pool)
    .await
    .map_err(|e| e.to_string())?;

    let Some(row) = row else {
        return Ok(None);
    };

    let tags = load_tags(pool, row.id).await.unwrap_or_default();
    Ok(Some(Post {
        id: Some(row.id),
        slug: row.slug,
        title: row.title,
        summary: row.summary,
        body_markdown: row.body_markdown,
        tags,
        published_at: row.published_at.map(|t| t.date_naive().to_string()),
    }))
}

#[cfg(feature = "ssr")]
async fn load_tags(pool: &sqlx::PgPool, post_id: i64) -> Result<Vec<String>, String> {
    let tags = sqlx::query_scalar::<_, String>(
        r#"
        SELECT t.name
        FROM tags t
        INNER JOIN post_tags pt ON pt.tag_id = t.id
        WHERE pt.post_id = $1
        ORDER BY t.name
        "#,
    )
    .bind(post_id)
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(tags)
}

#[cfg(feature = "ssr")]
fn parse_tags(raw: &str) -> Vec<String> {
    raw.split(',')
        .map(|t| t.trim().to_string())
        .filter(|t| !t.is_empty())
        .collect()
}

#[cfg(feature = "ssr")]
fn normalize_slug(slug: &str) -> String {
    slug.trim().trim_matches('/').to_lowercase()
}

#[cfg(feature = "ssr")]
async fn replace_post_tags(
    pool: &sqlx::PgPool,
    post_id: i64,
    tags: &[String],
) -> Result<(), String> {
    sqlx::query("DELETE FROM post_tags WHERE post_id = $1")
        .bind(post_id)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

    for name in tags {
        let tag_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO tags (name) VALUES ($1)
            ON CONFLICT (name) DO UPDATE SET name = EXCLUDED.name
            RETURNING id
            "#,
        )
        .bind(name)
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

        sqlx::query(
            r#"
            INSERT INTO post_tags (post_id, tag_id) VALUES ($1, $2)
            ON CONFLICT DO NOTHING
            "#,
        )
        .bind(post_id)
        .bind(tag_id)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg(feature = "ssr")]
pub(crate) async fn fetch_all_posts(pool: &sqlx::PgPool) -> Result<Vec<Post>, String> {
    #[derive(sqlx::FromRow)]
    struct Row {
        id: i64,
        slug: String,
        title: String,
        summary: Option<String>,
        body_markdown: String,
        published_at: Option<chrono::DateTime<chrono::Utc>>,
    }

    let rows = sqlx::query_as::<_, Row>(
        r#"
        SELECT id, slug, title, summary, body_markdown, published_at
        FROM posts
        ORDER BY updated_at DESC, id DESC
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    let mut posts = Vec::with_capacity(rows.len());
    for row in rows {
        let tags = load_tags(pool, row.id).await.unwrap_or_default();
        posts.push(Post {
            id: Some(row.id),
            slug: row.slug,
            title: row.title,
            summary: row.summary,
            body_markdown: row.body_markdown,
            tags,
            published_at: row.published_at.map(|t| t.date_naive().to_string()),
        });
    }
    Ok(posts)
}

#[cfg(feature = "ssr")]
pub(crate) async fn fetch_post_by_id(
    pool: &sqlx::PgPool,
    id: i64,
) -> Result<Option<Post>, String> {
    #[derive(sqlx::FromRow)]
    struct Row {
        id: i64,
        slug: String,
        title: String,
        summary: Option<String>,
        body_markdown: String,
        published_at: Option<chrono::DateTime<chrono::Utc>>,
    }

    let row = sqlx::query_as::<_, Row>(
        r#"
        SELECT id, slug, title, summary, body_markdown, published_at
        FROM posts
        WHERE id = $1
        LIMIT 1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(|e| e.to_string())?;

    let Some(row) = row else {
        return Ok(None);
    };
    let tags = load_tags(pool, row.id).await.unwrap_or_default();
    Ok(Some(Post {
        id: Some(row.id),
        slug: row.slug,
        title: row.title,
        summary: row.summary,
        body_markdown: row.body_markdown,
        tags,
        published_at: row.published_at.map(|t| t.date_naive().to_string()),
    }))
}

/// Admin: list all posts including drafts.
#[server(AdminListPosts)]
pub async fn admin_list_posts() -> Result<Vec<PostSummary>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use super::auth_http::require_pool;
        let _ = crate::server::require_admin().await?;
        let pool = require_pool()?;
        let posts = fetch_all_posts(&pool).await.map_err(ServerFnError::new)?;
        Ok(posts.iter().map(PostSummary::from).collect())
    }
    #[cfg(not(feature = "ssr"))]
    {
        Ok(Vec::new())
    }
}

#[server(AdminGetPost)]
pub async fn admin_get_post(id: i64) -> Result<Post, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use super::auth_http::require_pool;
        let _ = crate::server::require_admin().await?;
        let pool = require_pool()?;
        fetch_post_by_id(&pool, id)
            .await
            .map_err(ServerFnError::new)?
            .ok_or_else(|| ServerFnError::new("文章不存在"))
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = id;
        Err(ServerFnError::new("ssr only"))
    }
}

#[server(AdminCreatePost)]
pub async fn admin_create_post(input: PostInput) -> Result<Post, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use super::auth_http::require_pool;
        use chrono::Utc;

        let _ = crate::server::require_admin().await?;
        let pool = require_pool()?;
        let slug = normalize_slug(&input.slug);
        if slug.is_empty() || input.title.trim().is_empty() {
            return Err(ServerFnError::new("标题与 slug 不能为空"));
        }
        let published_at = if input.publish {
            Some(Utc::now())
        } else {
            None
        };
        let summary = {
            let s = input.summary.trim();
            if s.is_empty() {
                None
            } else {
                Some(s.to_string())
            }
        };
        let id: i64 = sqlx::query_scalar(
            r#"
            INSERT INTO posts (slug, title, summary, body_markdown, published_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id
            "#,
        )
        .bind(&slug)
        .bind(input.title.trim())
        .bind(&summary)
        .bind(&input.body_markdown)
        .bind(published_at)
        .fetch_one(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        let tags = parse_tags(&input.tags);
        replace_post_tags(&pool, id, &tags)
            .await
            .map_err(ServerFnError::new)?;
        fetch_post_by_id(&pool, id)
            .await
            .map_err(ServerFnError::new)?
            .ok_or_else(|| ServerFnError::new("创建后读取失败"))
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = input;
        Err(ServerFnError::new("ssr only"))
    }
}

#[server(AdminUpdatePost)]
pub async fn admin_update_post(id: i64, input: PostInput) -> Result<Post, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use super::auth_http::require_pool;
        use chrono::Utc;

        let _ = crate::server::require_admin().await?;
        let pool = require_pool()?;
        let slug = normalize_slug(&input.slug);
        if slug.is_empty() || input.title.trim().is_empty() {
            return Err(ServerFnError::new("标题与 slug 不能为空"));
        }
        let published_at = if input.publish {
            // Keep existing published_at if already published; else now.
            let existing = fetch_post_by_id(&pool, id)
                .await
                .map_err(ServerFnError::new)?;
            let Some(existing) = existing else {
                return Err(ServerFnError::new("文章不存在"));
            };
            if existing.published_at.is_some() {
                // leave as-is in SQL by reading current timestamp from DB when publish stays true
                sqlx::query_scalar::<_, Option<chrono::DateTime<chrono::Utc>>>(
                    "SELECT published_at FROM posts WHERE id = $1",
                )
                .bind(id)
                .fetch_one(&pool)
                .await
                .map_err(|e| ServerFnError::new(e.to_string()))?
                .or_else(|| Some(Utc::now()))
            } else {
                Some(Utc::now())
            }
        } else {
            None
        };
        let summary = {
            let s = input.summary.trim();
            if s.is_empty() {
                None
            } else {
                Some(s.to_string())
            }
        };
        let result = sqlx::query(
            r#"
            UPDATE posts
            SET slug = $2, title = $3, summary = $4, body_markdown = $5, published_at = $6
            WHERE id = $1
            "#,
        )
        .bind(id)
        .bind(&slug)
        .bind(input.title.trim())
        .bind(&summary)
        .bind(&input.body_markdown)
        .bind(published_at)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
        if result.rows_affected() == 0 {
            return Err(ServerFnError::new("文章不存在"));
        }
        let tags = parse_tags(&input.tags);
        replace_post_tags(&pool, id, &tags)
            .await
            .map_err(ServerFnError::new)?;
        fetch_post_by_id(&pool, id)
            .await
            .map_err(ServerFnError::new)?
            .ok_or_else(|| ServerFnError::new("更新后读取失败"))
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = (id, input);
        Err(ServerFnError::new("ssr only"))
    }
}

#[server(AdminDeletePost)]
pub async fn admin_delete_post(id: i64) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use super::auth_http::require_pool;
        let _ = crate::server::require_admin().await?;
        let pool = require_pool()?;
        let result = sqlx::query("DELETE FROM posts WHERE id = $1")
            .bind(id)
            .execute(&pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;
        if result.rows_affected() == 0 {
            return Err(ServerFnError::new("文章不存在"));
        }
        Ok(())
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = id;
        Err(ServerFnError::new("ssr only"))
    }
}

#[cfg(all(test, feature = "ssr"))]
mod tests {
    use super::*;
    use crate::domain::seed_posts;
    use crate::server::test_db::shared_pool;

    #[test]
    fn seed_includes_plain_start() {
        let posts = seed_posts();
        assert!(posts.iter().any(|p| p.slug == "plain-start"));
        let summary = PostSummary::from(&posts[0]);
        assert_eq!(summary.slug, "plain-start");
        assert!(!summary.title.is_empty());
    }

    #[test]
    fn markdown_renders_heading() {
        let html = markdown_to_html("## 技术选型\n\nhello");
        assert!(html.contains("<h2>"));
        assert!(html.contains("技术选型"));
        assert!(html.contains("<p>hello</p>"));
    }

    #[test]
    fn to_detail_embeds_html() {
        let post = seed_posts().into_iter().next().unwrap();
        let detail = to_detail(post);
        assert!(detail.body_html.contains("<h2>") || detail.body_html.contains("<p>"));
    }

    #[tokio::test]
    async fn db_list_published_when_configured() {
        let Some(pool) = shared_pool().await else {
            eprintln!("skip db_list_published_when_configured: DATABASE_URL unavailable");
            return;
        };

        let result = fetch_published_posts(pool).await;
        assert!(
            result.is_ok(),
            "fetch_published_posts failed (apply sql/posts.sql?): {:?}",
            result.err()
        );
    }

    #[tokio::test]
    async fn db_get_plain_start_when_seeded() {
        let Some(pool) = shared_pool().await else {
            eprintln!("skip db_get_plain_start_when_seeded: DATABASE_URL unavailable");
            return;
        };

        let post = fetch_post_by_slug(pool, "plain-start")
            .await
            .expect("query failed — apply sql/posts.sql + sql/seed_posts.sql?");
        // OK if None: schema exists but seed not applied yet.
        if let Some(post) = post {
            assert_eq!(post.slug, "plain-start");
            assert!(!post.body_markdown.is_empty());
        }
    }
}
