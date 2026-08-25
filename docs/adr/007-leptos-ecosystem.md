# ADR-007: Leptos ecosystem and advanced patterns

## Status

Accepted

## Context

Leptos has a rich ecosystem ([awesome-leptos](https://github.com/leptos-rs/awesome-leptos)). Reimplementing hooks, UI primitives, and async boundaries by hand increases WASM size and maintenance cost. The site should exploit SSR streaming, Suspense/Transition, fine-grained signals, and eventually islands.

## Decision

1. **Prefer ecosystem crates** when they fit: e.g. **thaw** (Leptos 0.8 → thaw `0.5` line), **leptos-use** for browser/reactive utilities.
2. **Async data**: load via server functions / `Resource`; wrap UI in `<Suspense>` / `<Transition>`; avoid blocking the whole page shell.
3. **Islands direction**: migrate interactive islands toward `islands` / islands-router so most of the site stays SSR HTML; full-page `hydrate` is the current scaffold, not the long-term ideal for every page.
4. **Thaw + Tailwind**: Thaw for complex interactive controls; Tailwind + `tokens.css` for layout, chrome, and extreme cyberpunk skin. One token source of truth for brand colors.
5. Before adding a crate, check awesome-leptos + crates.io compatibility with Leptos **0.8** and musl-friendly deps.

## Consequences

- New UI work should check thaw/leptos-use before inventing primitives
- Agents should reach for Suspense/Transition when introducing async fetches
- Island adoption may change `lib` features over time; document in cargo-workflow skill when switched
