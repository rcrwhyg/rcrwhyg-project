# Architecture — 如春日午后阳光

Personal site: **Leptos 0.8 SSR-first**, **Axum** (HTTP + planned WebSocket/SSE), **cargo-leptos**, **Tailwind**, optional **Postgres/SQLx**. Production server binary is a **static musl** artifact via **cargo-zigbuild**.

## Product shape

- **Diversified site areas** — not limited to blog + tools. New sections register like tools and share one route tree + one chrome.
- Current areas: home, blog/essays (`posts`), toolbox (`/tools` + registry)
- **Single UI chrome** (fixed header + content). Themes: **dark** (default) + **light** only via `data-theme` (ADR-003)
- Visual: **extreme cyberpunk** dynamic backdrop (grid/orbs/beams + mouse-follow glow) — Chinese-first; not pixel-primary (ADR-004)
- Export/import: Markdown frontmatter in `src/domain/export.rs` (no third-party sync yet)

## Rendering & interactivity

| Principle | Practice |
|-----------|----------|
| SSR-first | HTML from server; SEO and first paint from SSR |
| Prefer islands over full-page hydrate long-term | Interactive widgets as `#[island]` / islands-router; keep WASM small on 2C2G |
| Async UI | `Resource` / server functions + `<Suspense>` / `<Transition>` for loading boundaries |
| Fine-grained reactivity | Signals; avoid coarse re-render patterns |
| Ecosystem | Prefer [awesome-leptos](https://github.com/leptos-rs/awesome-leptos) crates: **thaw** (UI), **leptos-use** (hooks), etc. |

Thaw + Tailwind: use Thaw for interactive controls; keep site chrome/tokens in Tailwind + CSS variables. Do not fork a second design system.

**Thaw pin:** use git `https://github.com/thaw-ui/thaw` until crates.io `0.5.0-beta` builds cleanly on rustc ≥ 1.97. Project enables `erase_components` via `.cargo/config.toml`.

## Transport (Axum)

Axum is the **process edge**, not “HTTP only”:

| Channel | Use |
|---------|-----|
| HTTP | Pages, REST-ish APIs, server functions, `/health` |
| WebSocket | Bidirectional realtime — stub at `/ws/echo` |
| SSE | One-way server push — stub at `/sse/heartbeat` |

Prefer nesting `/api`, `/ws`, `/sse` beside `leptos_routes`. Shared state via Axum `State` / Leptos context. All realtime code stays behind `ssr`.

## Module map (target)

```text
src/
  app/           # App router, theme, layout, CyberBackground
  pages/         # route views per site area
  components/    # shared UI (+ thaw wrappers over time)
  domain/        # Post, ToolMeta, SiteArea, export/import
  tools/         # tool registry
  areas/         # (planned) site-area registry beyond blog/tools
  server/        # AppState, #[server], /health, SSE, WS (ssr)
  main.rs        # Axum bootstrap
  lib.rs         # hydrate / islands client entry
style/
  tokens.css
  tailwind.css
sql/
  posts.sql      # canonical blog schema
  articles.sql   # legacy — do not extend
docs/
  architecture.md
  build-musl.md
  adr/
```

## Routes (current)

| Path | Page |
|------|------|
| `/` | Home |
| `/blog` | Blog list (`list_published_posts`) |
| `/blog/:slug` | Post detail (`get_post_by_slug` + Markdown→HTML) |
| `/admin` | Solo admin dashboard (session required) |
| `/admin/login` | Admin login |
| `/admin/posts` | Post list / CRUD (session required) |
| `/admin/posts/new` | Create post |
| `/admin/posts/:id/edit` | Edit post |
| `/tools` | Tool index |
| `/tools/echo` | Example tool |

One route tree only. New areas add routes + header nav + registry entries.

## Build & deploy (locked)

**Goal:** pure SSR delivery + **static musl server binary** for the VPS.

1. Front assets / WASM islands: `cargo leptos build --release` → `target/site`
2. Server binary (static): `cargo zigbuild --release --target x86_64-unknown-linux-musl --features ssr`  
   (or `aarch64-unknown-linux-musl` if the VPS is ARM)
3. Ship: musl binary + `site/` directory; no glibc runtime on the server required for the binary itself

See [build-musl.md](build-musl.md). Prefer crates that link cleanly on musl (avoid glibc-only native deps; prefer pure Rust TLS like `rustls`).

## Environment

See [`.env.example`](../.env.example). Never commit secrets.

| Variable | Purpose |
|----------|---------|
| `DATABASE_URL` | Optional Postgres URL; when unset/unavailable, blog falls back to in-memory seed posts |
| `LEPTOS_*` | Usually from `Cargo.toml` metadata |

Blog content path: apply `sql/posts.sql` (+ optional `sql/seed_posts.sql`), or rely on `domain::seed_posts` without a database.

Local `.env` `DATABASE_URL` is the supported way to develop against real Postgres; the same URL feeds soft-gated DB tests ([testing.md](testing.md)).

## Testing

See [testing.md](testing.md). Coverage is **thin by default**; agents must extend tests with features (especially before exposing write/admin APIs — ADR-011).

### Runtime endpoints (stubs)

| Path | Kind |
|------|------|
| `/health` | JSON liveness + db probe (`connected` / `unset` / `error`) |
| `/sse/heartbeat` | SSE tick stream |
| `/ws/echo` | WebSocket echo |

## 2C2G ops

- Single process: HTTP + WS + SSE in one Axum app
- Keep island/WASM payloads small; CRT effects = CSS
- No microservices

## Security

- No secrets in source or public routes
- Server-only crates: `optional` + `ssr`
- WS/SSE: authenticate/rate-limit when exposing mutating channels

## ADR index

| ADR | Topic |
|-----|-------|
| 001 | Leptos + Axum stack |
| 002 | Posts-first content |
| 003 | Single chrome + dark/light themes |
| 004 | Design tokens (extreme cyberpunk) |
| 005 | Tool registry |
| 006 | Diversified site areas |
| 007 | Leptos ecosystem (thaw, leptos-use, Suspense, islands) |
| 008 | WebSocket + SSE on Axum |
| 009 | SSR musl / zigbuild |
| 010 | ~~Terminal CLI~~ — **superseded** by 003 |
| 011 | Solo-author publishing (proposed) |
