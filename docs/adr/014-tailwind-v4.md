# ADR-014: Tailwind CSS v4 (CSS-native config)

## Status

Accepted (2026-08-31)

## Context

- cargo-leptos 0.3.7 defaults to downloading **Tailwind v4** standalone CLI.
- The site evolved on **v3** patterns: `@tailwind` directives, `tailwind.config.js` with Leptos-specific `content.extract` / `safelist`, and `@layer components` chrome in `style/tailwind.css`.
- Pinning **v3.4.0** on CI (v0.3.10) fixed production CSS but diverged from cargo-leptos defaults and blocked “latest toolchain” intent.
- Tailwind v4 removes JS `safelist` and `@config` bridge support for content transforms; configuration moves to CSS (`@import`, `@source`, `@source inline`).

## Decision

1. **Tailwind v4.3.x only** — local dev and CD use the v4 standalone CLI (or cargo-leptos download of the same version).
2. **Remove `tailwind.config.js`** — drop `tailwind-config-file` from `[package.metadata.leptos]`.
3. **CSS entry** (`style/tailwind.css`):
   - `@import "tailwindcss";`
   - `@source "../src/**/*.rs";` + `@source "./tailwind.safelist.html";`
   - Keep site chrome in `@layer base` / `@layer components`; tokens stay in `style/tokens.css`.
4. **Leptos class conventions**:
   - `<A>` (leptos_router) uses `attr:class=` only — not `class=`.
   - Static utilities on other elements use `class="..."`.
   - No `format!()` for Tailwind/component class strings — use static `match` maps.
5. **Release gate** — `tools/check-site-css.sh` after `cargo leptos build --release` (CD + local verification).

## Consequences

- CD installs `tailwindcss-linux-x64` v4.3.3 on PATH before build; no Node/npm Tailwind v3 step.
- Developers with a global **v3** `tailwindcss` on PATH must upgrade or remove it (v3 CLI cannot parse `@import "tailwindcss"`).
- Future dynamic classes: extend `style/tailwind.safelist.html` or `@source inline("utility")` — not JS safelist.

## References

- [Tailwind v4 upgrade guide](https://tailwindcss.com/docs/upgrade-guide)
- [Detecting classes / @source](https://tailwindcss.com/docs/detecting-classes-in-source-files)
- Project article: `articles/2c2g-server/06-ai-collab-engineering-lessons.md` (Tailwind purge section)
