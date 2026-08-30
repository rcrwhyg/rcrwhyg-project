# Architecture — 如春日午后阳光

Personal site: **Leptos 0.8 SSR-first**, **Axum**, **cargo-leptos**, **Tailwind CSS v4**, optional **Postgres/SQLx** (auth only). Production server binary is a **static musl** artifact via **cargo-zigbuild**.

## Product shape

- **Diversified site areas** — home hub, articles, tools, radar, lab, about, music, clock; one route tree + one chrome.
- **Single visual theme** — fixed **dark** mint/sky palette (`data-theme="dark"`); no user toggle.
- Visual: mint + sky **calm-tech** gradient backdrop with drifting particles; **0.30 glass** surfaces (`--chrome-bg`).
- Content: **`articles/`** is the only publish path (git + CD). Admin session is for ops, not in-browser editing.

## Rendering & interactivity

| Principle | Practice |
|-----------|----------|
| SSR-first | HTML from server; SEO and first paint from SSR |
| Prefer islands long-term | Interactive widgets as islands; keep WASM small on 2C2G |
| Async UI | `Resource` + server functions + `<Suspense>` |
| Ecosystem | **thaw** (UI), **leptos-use** (hooks); site chrome in Tailwind + `style/tokens.css` |

**Thaw pin:** git `https://github.com/thaw-ui/thaw` until crates.io `0.5.0-beta` builds on rustc ≥ 1.97.

## Transport (Axum)

| Channel | Use |
|---------|-----|
| HTTP | Pages, server functions, `/health` |
| WebSocket | Stub `/ws/echo` |
| SSE | Stub `/sse/heartbeat` |

Shared state via Axum `State` / Leptos context. Realtime behind `ssr` feature.

## Module map

```text
src/
  app/           # router, theme, layout, DynamicBackground, admin_session
  pages/         # route views per area
  components/    # BeianFooter, …
  domain/        # export/import helpers (legacy Post types)
  tools/         # tool registry
  server/        # articles, auth, area data loaders, markdown, /health, SSE, WS
  bin/create-admin.rs
  main.rs
style/
  tokens.css     # mint/sky dark tokens (+ light palette reserved for palette-preview)
  tailwind.css   # @import tailwindcss + @source + @layer chrome (ADR-014)
  tailwind.safelist.html  # responsive / arbitrary utility safelist for release builds
articles/        # canonical Markdown (合集子目录 + NN-slug.md)
data/            # radar.json, music.json, lab.json
content/         # about.md
sql/
  auth.sql       # admin sessions (Postgres)
docs/
```

## Routes (current)

| Path | Page |
|------|------|
| `/` | Home hub (carousel + recent articles) |
| `/articles` | Article index (合集 + 随笔) |
| `/articles/:slug` | Article detail (Markdown → HTML) |
| `/blog`, `/blog/:slug` | **308** → `/articles` (legacy) |
| `/tools`, `/tools/echo` | Toolbox |
| `/radar` | Learning radar (`data/radar.json`) |
| `/lab` | Lab demos (`data/lab.json`) |
| `/about` | About (`content/about.md`) |
| `/music` | Playlist (`data/music.json`) |
| `/clock` | Pomodoro timer |
| `/admin` | Admin dashboard (session) |
| `/admin/login` | Admin login |

Logged-in admin sees **后台** + **退出** in header; public nav unchanged.

## Content model (articles)

| Topic | Choice |
|-------|--------|
| Source of truth | `articles/<合集>/NN-slug.md` or根目录 `NN-slug.md`（随笔） |
| Collection meta | `articles/<合集>/_meta.json`（`title`, 可选 `placeholder`） |
| Deploy | CD ships entire `articles/` tree + `data/` + `content/` into site root |
| Index UI | Flat list, 倒序；右上角徽章显示合集名，无合集为「随笔」 |
| Sort | File number desc → date desc → slug |
| Exclude | `README.md`, `templates/`, non-`NN-slug` names, `> **站点发布**: 否` |
| Admin | CLI `create-admin`; session for `/admin` ops — **no** browser article CRUD |

Legacy `sql/posts.sql` is deprecated; do not extend.

## Theme & layout

- **Dark only**: `data-theme="dark"` on `html` / `body` / `.site-root`; Thaw `Theme::dark()`.
- Tokens in `style/tokens.css`. No theme toggle, no `localStorage` preference.
- Layout: `.site-root` flex column (`min-height: 100dvh`); fixed header; `.site-main` `padding-top: var(--site-header-h)`; footer `flex-shrink: 0` stays in viewport.

## Build & deploy

1. `cargo leptos build --release` → `target/site`
2. `cargo zigbuild --release --target x86_64-unknown-linux-musl --features ssr`
3. Ship musl binary + site assets + `articles/` + `data/` + `content/`. See [build-musl.md](build-musl.md), [deploy-vps.md](deploy-vps.md).

## Environment

See [`.env.example`](../.env.example).

| Variable | Purpose |
|----------|---------|
| `DATABASE_URL` | Postgres for **admin auth** + soft-gated tests |
| `COOKIE_SECURE` | `false` local HTTP; `true` behind Caddy HTTPS |
| `SESSION_TTL_HOURS` | Admin session lifetime |
| `RATE_LIMIT_*` | Global + auth rate limits |

Articles do **not** require Postgres.

## Testing

See [testing.md](testing.md). Article static checks: `./tools/check-articles.sh` (recursive under `articles/`).

## Runtime endpoints

| Path | Kind |
|------|------|
| `/health` | JSON liveness + db probe |
| `/sse/heartbeat` | SSE tick |
| `/ws/echo` | WebSocket echo |

## Related ADRs

- [002-content-posts-first.md](adr/002-content-posts-first.md) — **superseded** by articles file model
- [003-dual-shell.md](adr/003-dual-shell.md) — single chrome; **dark-only** theme (no toggle)
- [004-design-tokens.md](adr/004-design-tokens.md) — calm-tech tokens
- [006-diversified-site-areas.md](adr/006-diversified-site-areas.md)
- [011-solo-author-publishing.md](adr/011-solo-author-publishing.md) — admin auth; publishing via git
- [012-cd-pipeline-github-actions.md](adr/012-cd-pipeline-github-actions.md) — CD ships `articles/` subtree
