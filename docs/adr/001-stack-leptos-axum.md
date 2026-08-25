# ADR-001: Keep Leptos 0.8 + Axum + cargo-leptos

## Status

Accepted

## Context

The repo already scaffolds Leptos SSR/hydrate with Axum and cargo-leptos. The site runs on a 2C2G VPS where memory and binary size matter. SEO for blog content benefits from SSR.

## Decision

Continue with Leptos 0.8, Axum, cargo-leptos, Tailwind, and optional SQLx/Postgres under the `ssr` feature. Do not split into a separate frontend SPA framework.

## Consequences

- Dual compile (native SSR + client WASM for hydrate/islands) remains during development
- **Production server binary** targets static **musl** via **cargo-zigbuild** (see ADR-009)
- Axum also owns future **WebSocket** and **SSE** routes (see ADR-008)
- Prefer Leptos ecosystem crates (thaw, leptos-use, …) per ADR-007
- Server deps must stay `optional` + feature-gated
- Agent skills for Leptos stay project-scoped; Rust/Axum/Tokio skills can be global
