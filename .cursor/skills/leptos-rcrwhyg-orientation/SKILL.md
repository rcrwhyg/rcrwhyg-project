---
name: leptos-rcrwhyg-orientation
description: Orients agents in the rcrwhyg Leptos personal site—module map, ssr/hydrate features, diversified areas, ADR index, musl deploy. Use when working in this repository, editing Leptos components, or when the user mentions 如春日午后阳光, rcrwhyg, or this Cargo project.
---

# rcrwhyg Leptos Orientation

Read [docs/architecture.md](../../../docs/architecture.md) and ADRs under [docs/adr/](../../../docs/adr/) before large changes.

## Product

Diversified personal site: articles, tools, radar, lab, and **future areas**. Single chrome shell, **fixed dark theme**, calm-tech dynamic backdrop (CJK-first), SSR-first, static musl deploy. **No Terminal mode.**

## Map

| Path | Role |
|------|------|
| `src/main.rs` | Axum bootstrap: Leptos + `/health` + `/sse/*` + `/ws/*` |
| `src/lib.rs` | hydrate / islands client entry |
| `src/app/` | App, shell HTML, theme, layout, `DynamicBackground` |
| `src/pages/` | Route views per site area |
| `src/components/` | Shared UI (footer, …) |
| `src/domain/` | export/import, legacy Post types |
| `src/server/` | `AppState`, articles/auth/area loaders, markdown, health/SSE/WS (ssr) |
| `src/tools/registry.rs` | Toolbox registry |
| `articles/` | Canonical Markdown (collections + essays) |
| `style/tokens.css` | Design tokens (dark production; light preview-only) |
| `docs/build-musl.md` | zigbuild musl release |
| `docs/testing.md` | Unit / DB integration / e2e policy |

## Features

- `ssr` — server binary (axum, sqlx, tokio, ws/sse handlers)
- `hydrate` — current WASM client scaffold; long-term prefer **islands** (ADR-007)

Never add server-only crates without `optional` + `ssr`. Prefer musl-friendly deps (`rustls`).

## Testing

Read [docs/testing.md](../../../docs/testing.md). Coverage is intentionally thin today; extend with features. DB tests share `.env` `DATABASE_URL` via `server::test_db::shared_pool`.

## Related project skills

- UI / theme / cyber bg → `leptos-ui-theme-chrome`
- Areas / articles / tools / export → `leptos-content-and-tools`
- thaw / leptos-use / Suspense / islands → `leptos-ecosystem-patterns`
- Build / watch / musl / tests → `leptos-cargo-workflow`
- Solo publishing / admin auth → ADR-011 (`sql/auth.sql`, `/admin/*`, rate limits)
- Article workflow → `rcrwhyg-article-workflow`

## Global skills

`rust-orientation`, `rust-axum-tokio` (HTTP+WS+SSE), `rust-sqlx-postgres`
