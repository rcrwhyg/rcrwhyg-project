---
name: leptos-ecosystem-patterns
description: Guides Leptos 0.8 ecosystem usage for rcrwhyg—thaw UI, leptos-use, Suspense/Transition, Resources, islands, and awesome-leptos discovery. Use when adding UI components, async data loading, islands, hooks, or when the user mentions thaw, leptos-use, Suspense, Transition, or islands.
---

# Leptos ecosystem patterns (rcrwhyg)

Follow ADR-007. Discover crates via [awesome-leptos](https://github.com/leptos-rs/awesome-leptos); verify **Leptos 0.8** compatibility and musl-friendly deps.

## Preferred libraries

| Need | Prefer |
|------|--------|
| Interactive UI kit | **thaw** (0.5 line for Leptos 0.8) |
| Browser / reactive hooks | **leptos-use** |
| Layout / cyberpunk chrome + backdrop | Tailwind + `style/tokens.css` + `CyberBackground` |

Do not invent a second component library. Wrap thaw where theming must match tokens.

## Async data

1. Fetch with `#[server]` / `Resource` (or equivalent 0.8 APIs)
2. Bound UI with `<Suspense>` and/or `<Transition>`
3. Keep chrome (nav/footer) outside the suspending region when possible
4. Prefer streaming SSR when the page benefits from progressive HTML

## Reactivity

- Use fine-grained signals; avoid replacing large view trees when a signal update suffices
- Derive with `Memo` / `Signal::derive` when computing from multiple sources

## Islands direction

- Long-term: interactive pieces as islands; most content SSR-only HTML
- Current scaffold may still use full `hydrate` — new heavy widgets should be designed island-friendly (small client surface)
- Do not ship large client-only SPAs inside pages

## Thaw + Tailwind

- Thaw for forms, dialogs, menus, complex controls
- Tailwind utilities + CSS variables for page layout and cyberpunk skin
- Brand colors only via tokens (`--accent`, etc.)

## In this repo

- Theme persistence: `leptos_use::storage::use_local_storage` in `src/app/theme.rs`
- Thaw: git `thaw-ui/thaw` (crates.io beta hits rustc 1.97 query-depth overflow) + `SSRMountStyleProvider` / `ConfigProvider` / `Button`
- Build: `.cargo/config.toml` sets `--cfg erase_components`; `lib.rs` has `#![recursion_limit = "512"]`
- Suspense demo: `src/pages/tools.rs` + `src/server/ping.rs` (`server_ping`, `db_status`)
- SSE stub: `GET /sse/heartbeat`
- WS stub: `GET /ws/echo`
- Health: `GET /health` + optional `AppState.db` (`DATABASE_URL`)
- Pool injection: `leptos_routes_with_context` → `provide_context(PgPool)` when connected

## Before adding a crate

1. Check awesome-leptos + docs for Leptos 0.8
2. Confirm it does not pull glibc-only / OpenSSL-system deps that break musl zigbuild
3. Feature-gate server-only pieces under `ssr`
4. For thaw bumps: prefer git main until crates.io ships recursion_limit + rustc 1.97 fixes
