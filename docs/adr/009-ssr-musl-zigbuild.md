# ADR-009: SSR-first + static musl via cargo-zigbuild

## Status

Accepted

## Context

Deploy target is a small Linux VPS. We want a **self-contained server binary** without relying on host glibc, and a delivery model centered on **SSR** (HTML from server), not a CSR-only SPA. Cross-compilation from macOS/dev machines should be reliable.

## Decision

1. **Runtime product**: SSR site served by a single Axum binary + `site/` assets (CSS, optional island WASM/JS).
2. **Server link target**: `*-unknown-linux-musl` static binary.
3. **Toolchain**: [cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild) (`cargo zigbuild`) for cross-compile to musl.
4. **Asset pipeline**: still use `cargo leptos build --release` for the site package; pair with zigbuild for the native server binary when shipping.
5. Prefer pure-Rust / musl-friendly native deps (`rustls`, avoid OpenSSL-system coupling).

## Consequences

- Documented in `docs/build-musl.md` and `leptos-cargo-workflow` skill
- CI/release scripts should produce musl binary + `site/`
- Agents must not assume glibc-only deploy or Docker-with-distro-libs as the primary path
