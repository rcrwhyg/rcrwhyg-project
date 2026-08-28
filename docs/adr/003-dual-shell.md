# ADR-003: Single chrome shell + dark/light themes

## Status

Accepted (supersedes dual Modern/Terminal shell decision)

## Context

Terminal / dual-shell UX added complexity without matching the Chinese-first product focus. The site needs one clear chrome and only two appearance modes.

## Decision

- **One** Leptos route tree and **one** site chrome (fixed header + content)
- Appearance modes: **`dark`** (default) and **`light` only** — no Terminal / Modern style switch
- Persist theme in `localStorage` key `rcrwhyg.theme`
- Reflect via `data-theme` on `html` / `body` / `.site-root`
- Dynamic calm-tech backdrop (`DynamicBackground`) with mouse-follow glow; tokens differ per theme (ADR-004)

## Consequences

- Terminal CLI, `StyleMode`, and related commands/session history are removed
- ADR-010 is superseded
- Nav lives only in the header for all pages
