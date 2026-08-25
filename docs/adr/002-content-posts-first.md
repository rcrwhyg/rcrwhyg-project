# ADR-002: Site-owned posts as content source of truth

## Status

Accepted

## Context

An earlier `articles` schema targeted multi-platform aggregation (WeChat, Zhihu, …). Product direction changed: this website is primary; cross-platform sync is deferred. Export/import should be possible later without locking the schema to foreign platforms.

## Decision

- Canonical table/model: `posts` (+ tags) in `sql/posts.sql` and `domain::Post`
- Do not extend `sql/articles.sql`
- Provide `ContentExporter` / `ContentImporter` with a Markdown frontmatter intermediate format
- No WeChat or other platform adapters in this phase

## Consequences

- Agents must model content as site-owned posts
- Platform adapters, if added later, map through the Markdown/DTO layer
