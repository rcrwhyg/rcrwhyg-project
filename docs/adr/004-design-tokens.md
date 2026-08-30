# ADR-004: Design tokens — calm-tech (dark)

## Status

Accepted (2026-08 update: production site is **dark-only**; light palette retained in tokens file for design preview only)

## Context

Chinese-first UI. Pixel fonts are abandoned. Product look is **calm-tech** with mint + sky accents on a deep green-black base.

## Decision

- Tokens in `style/tokens.css` (`--accent`, `--accent-2`, `--cyber-*`, `--site-header-h`, etc.)
- Typography: **Noto Sans SC** body; **Space Grotesk** display; **JetBrains Mono** for code/mono accents
- **Production theme (dark)**: deep green-black base, mint/sky accents, screen-blend cursor glow, 0.30 glass surfaces
- Dynamic backdrop layers (grid, beams, orbs, particles, vignette, mouse spotlight) in `.dynamic-bg*`
- UI chrome (header, panels, buttons, links) must use CSS variables — no hard-coded theme colors in Rust views
- Light token block in the same file is **not** exposed in the live site; reference only in `docs/palette-preview.html`

## Consequences

- Prefer `var(--*)` / utility classes tied to tokens
- Honor `prefers-reduced-motion` by disabling backdrop animations
- Do not reintroduce a theme toggle without a new ADR
