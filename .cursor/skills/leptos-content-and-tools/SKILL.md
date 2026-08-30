---
name: leptos-content-and-tools
description: Guides diversified site areas, articles model, tool registry, and Markdown export/import for rcrwhyg. Use when adding a new site section/area, articles, tools, registry entries, ContentExporter/Importer, or when the user mentions articles, collections, tools, gallery, notes, export, or import.
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

## Articles (canonical content)

- Source of truth: `articles/` in git — collection subdirs with `_meta.json` + `NN-slug.md`, or root-level essays
- Server: `src/server/articles.rs` — recursive scan, Markdown → HTML via `src/server/markdown.rs` (ssr-only)
- Routes: `/articles`, `/articles/:slug`; legacy `/blog` → 308 redirect
- Deploy: CD tars entire `articles/` tree into site root (see ADR-012)
- Exclude from index: `README.md`, `templates/`, non-`NN-slug` names, `> **站点发布**: 否`
- Sort: file number desc → date desc → slug
- **Write/publish path**: edit files in repo → CI → CD; admin session is ops-only (ADR-011)
- Do not extend `sql/posts.sql` or `sql/articles.sql` (legacy)
- WeChat repost is user-managed; AI does not publish externally

## Testing (content)

See [docs/testing.md](../../../docs/testing.md). New article parsing / listing logic should add unit coverage where practical.

## Export / import

- Traits in `src/domain/export.rs`
- Intermediate: Markdown frontmatter (`MarkdownBundle`)
- Adapters implement traits; pages must not call platform APIs directly

## Adding a tool

1. `ToolMeta` in `src/tools/registry.rs`
2. `Route` in `src/app/mod.rs`
3. Page component
4. Listed on `/tools` automatically
