# ADR-003: Single chrome shell + fixed dark theme

## Status

Accepted (supersedes dual Modern/Terminal shell decision; **2026-08** update: dark-only, no user toggle)

## Context

Terminal / dual-shell UX added complexity without matching the Chinese-first product focus. The site needs one clear chrome and a consistent calm-tech look. A dark/light toggle was tried and removed — product direction is **fixed dark**.

## Decision

- **One** Leptos route tree and **one** site chrome (fixed header + content + footer)
- Appearance: **`dark` only** — no light theme in production UI, no theme switch
- Reflect via `data-theme="dark"` on `html` / `body` / `.site-root` (SSR shell hard-coded)
- Thaw UI: `Theme::dark()` via `SitePreference`; no `localStorage` theme key
- Dynamic calm-tech backdrop (`DynamicBackground`) with mouse-follow glow; tokens in ADR-004
- Layout: flex column root; `.site-main` offsets fixed header with `padding-top: var(--site-header-h)`; footer stays visible via `flex-shrink: 0`

## Consequences

- Terminal CLI, `StyleMode`, `ThemeControls`, and related commands/session history are removed
- ADR-010 is superseded
- Nav lives only in the header for all pages
- Light token block in `style/tokens.css` may remain for `docs/palette-preview.html` only — not wired to the live site
