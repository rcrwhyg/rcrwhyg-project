---
name: leptos-ui-theme-chrome
description: Enforces single site chrome, fixed dark theme, and calm-tech dynamic backdrop for rcrwhyg. Use when editing layout, header, CyberBackground, CSS tokens, or when the user mentions dark mode, calm-tech UI, or mouse-follow effects.
---

# Theme + chrome (no Terminal)

## Rules

1. **One route tree** and **one chrome** (`SiteShell` / `SiteHeader` in `src/app/layout.rs`).
2. Theme: **fixed `dark` only**. Never reintroduce light mode or a theme toggle unless a new ADR says so.
3. Reflect with `data-theme="dark"` on `html` / `body` / `.site-root` (SSR shell hard-coded). Thaw: `Theme::dark()` via `SitePreference`.
4. Backdrop: `DynamicBackground` — animated grid/orbs/beams + **mouse-follow** spotlight. Pointer-events none; listen via `window_event_listener` under `hydrate`.
5. All chrome/panels/buttons must use tokens from `style/tokens.css` (ADR-004).
6. Layout: `.site-root` flex column (`min-height: 100dvh`); `.site-main` `padding-top: var(--site-header-h)` for fixed header; `.site-footer` `flex-shrink: 0`.
7. Honor `prefers-reduced-motion`.

## Do not

- Add `StyleMode`, terminal panel, CLI commands, `data-style`, or `ThemeControls`
- Persist theme in `localStorage`
- Put theme colors as hard-coded hex in Rust views
