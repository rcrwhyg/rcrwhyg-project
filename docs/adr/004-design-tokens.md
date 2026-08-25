# ADR-004: Design tokens — extreme cyberpunk (dark + light)

## Status

Accepted

## Context

Chinese-first UI. Pixel fonts are abandoned. Product look is **extreme cyberpunk**, with **dark and light** themes that each have distinct glow/motion treatment.

## Decision

- Tokens in `style/tokens.css` (`--accent`, `--accent-2`, `--cyber-*`, etc.)
- Typography: **Noto Sans SC** body; **Orbitron** display; **Share Tech Mono** accents
- Dark: deep purple/black, cyan + magenta neon, screen-blend cursor glow
- Light: lavender mist base, violet/magenta accents, multiply-blend cursor glow
- Dynamic backdrop layers (grid, beams, orbs, scan, vignette, mouse spotlight) in `.cyber-bg*`
- UI chrome (header, panels, buttons, links) must read correctly in **both** themes via CSS variables — no hard-coded theme-only colors in Rust views

## Consequences

- Prefer `var(--*)` / utility classes tied to tokens
- Honor `prefers-reduced-motion` by disabling backdrop animations
