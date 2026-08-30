# ADR-002: Site-owned posts as content source of truth

## Status

**Superseded** (2026-08) — canonical content is now **`articles/` Markdown** (git + CD). This ADR is kept for history.

## Context

An earlier `articles` schema targeted multi-platform aggregation (WeChat, Zhihu, …). Product direction changed: this website is primary; cross-platform sync is deferred. Export/import should be possible later without locking the schema to foreign platforms.

Later, a `posts` DB table was introduced for in-browser CRUD. That path was removed in favor of file-based `articles/` with collection subdirectories (`_meta.json` + `NN-slug.md`).

## Decision (historical)

- Canonical table/model: `posts` (+ tags) in `sql/posts.sql` and `domain::Post`
- Do not extend `sql/articles.sql`
- Provide `ContentExporter` / `ContentImporter` with a Markdown frontmatter intermediate format
- No WeChat or other platform adapters in this phase

## Current direction

- **Source of truth**: `articles/` in git — see [architecture.md](../architecture.md) and [011-solo-author-publishing.md](011-solo-author-publishing.md)
- `sql/posts.sql` is legacy; do not extend or use for new content
- WeChat【如春日午后阳光】 is a user-managed repost channel, not a second fact source

## Consequences

- Agents must model published essays as `articles/` files, not DB posts
- Platform adapters, if added later, map through the Markdown/DTO layer
