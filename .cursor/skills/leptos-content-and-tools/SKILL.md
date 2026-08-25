---
name: leptos-content-and-tools
description: Guides diversified site areas, posts model, tool registry, and Markdown export/import for rcrwhyg. Use when adding a new site section/area, blog posts, tools, registry entries, ContentExporter/Importer, or when the user mentions posts, tools, gallery, notes, export, or import.
---

# Content, tools, and site areas

## Diversified areas (ADR-006)

The site is **not** limited to blog + tools. New areas (notes, gallery, playgrounds, experiments, …) are in scope when they fit a personal site.

Checklist for a new area:

1. Domain types if persisted (under `src/domain/`)
2. Page(s) under `src/pages/` (or `src/areas/<name>/` when introduced)
3. `Route` in `src/app/mod.rs` (single router)
4. Nav: header links in `SiteHeader` (`src/app/layout.rs`)
5. Optional registry entry (mirror `tools/registry.rs`; planned `areas` registry)

## Posts

- Source of truth: `domain::Post` + `sql/posts.sql` (+ optional `sql/seed_posts.sql`)
- Server fns: `list_published_posts`, `get_post_by_slug` in `src/server/posts.rs`
- DB helpers `fetch_*(&PgPool)` are shared with soft-gated integration tests
- No DB / empty table → fall back to `domain::seed_posts`
- Detail HTML is rendered on the server with `pulldown-cmark` (ssr-only)
- Routes: `/blog`, `/blog/:slug`
- **Write/publish path**: solo admin via session cookie (ADR-011); create admin with `create-admin` CLI, then `/admin/login` + `/admin/posts`
- Rate limits: public + auth (see `docs/testing.md` / ADR-011)
- Do not extend `sql/articles.sql` (legacy)
- No WeChat/platform sync unless explicitly requested

## Testing (content)

See [docs/testing.md](../../../docs/testing.md). New post queries / mutate APIs must add unit and (when DB) integration coverage.

## Export / import

- Traits in `src/domain/export.rs`
- Intermediate: Markdown frontmatter (`MarkdownBundle`)
- Adapters implement traits; pages must not call platform APIs directly

## Adding a tool

1. `ToolMeta` in `src/tools/registry.rs`
2. `Route` in `src/app/mod.rs`
3. Page component
4. Listed on `/tools` automatically
