---
name: leptos-ui-theme-chrome
description: Enforces single site chrome, dark/light themes only, and calm-tech dynamic backdrop for rcrwhyg. Use when editing layout, header, theme toggle, CyberBackground, CSS tokens, or when the user mentions dark mode, light mode, calm-tech UI, or mouse-follow effects.
---

# Theme + chrome (no Terminal)

## Rules

1. **One route tree** and **one chrome** (`SiteShell` / `SiteHeader` in `src/app/layout.rs`).
2. Themes: **`dark` (default) and `light` only**. Never reintroduce Terminal / Modern style modes unless a new ADR says so.
3. Persist `rcrwhyg.theme` via `SitePreference`; reflect with `data-theme` on `html` / `body` / `.site-root`.
4. Backdrop: `CyberBackground` — animated grid/orbs/beams + **mouse-follow** spotlight. Pointer-events none; listen via `window_event_listener` under `hydrate`.
5. All chrome/panels/buttons must use tokens so **both** themes look intentional (ADR-004).
6. Content offset: `.site-body { padding-top: var(--site-header-h) }` under fixed header.
7. Honor `prefers-reduced-motion`.

## Do not

- Add `StyleMode`, terminal panel, CLI commands, or `data-style`
- Put theme colors as hard-coded hex in Rust views
