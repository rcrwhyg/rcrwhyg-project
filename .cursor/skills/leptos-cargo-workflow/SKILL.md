---
name: leptos-cargo-workflow
description: cargo-leptos watch/build plus static musl release via cargo-zigbuild for rcrwhyg. Use when running or debugging the site build, cross-compiling, end-to-end tests, deployment artifacts, or when the user mentions cargo leptos, zigbuild, musl, hydrate, or wasm-release.
---

# cargo-leptos + musl workflow

## Dev

```bash
cargo leptos watch
```

Metadata: `Cargo.toml` `[package.metadata.leptos]`.

Build notes:

- `.cargo/config.toml` enables `--cfg erase_components` (Leptos/thaw + rustc 1.97+)
- `src/lib.rs` sets `#![recursion_limit = "512"]`
- Thaw comes from **git** (`thaw-ui/thaw`), not crates.io beta
- **Tailwind CSS v4.3.x only** (`style/tailwind.css` uses `@import "tailwindcss"`). `cargo leptos` resolves `which tailwindcss` — PATH 上 v3 CLI 会导致构建失败。推荐 Homebrew：`brew install tailwindcss`（`/opt/homebrew/bin` 优先于 `/usr/local/bin` 的旧 npm v3）。与 CD 对齐版本：**v4.3.3**。

## Checks

```bash
cargo check --features ssr
cargo test --features ssr
cargo check --lib --features hydrate --target wasm32-unknown-unknown
# optional e2e after fixing end2end specs: cargo leptos end-to-end
```

Testing policy: [docs/testing.md](../../../docs/testing.md)

- Prefer unit tests with every domain/server change
- DB integration uses `.env` `DATABASE_URL` + shared pool (`src/server/test_db.rs`); soft-skip if unavailable
- Do not claim high coverage; Playwright scaffold is currently stale

## Release (locked goal)

1. Assets: `cargo leptos build --release` → `target/site`
2. Static server binary (see [docs/build-musl.md](../../../docs/build-musl.md)):

```bash
rustup target add x86_64-unknown-linux-musl
cargo zigbuild --release \
  --target x86_64-unknown-linux-musl \
  --features ssr \
  --bin rcrwhyg-server
```

Ship **musl binary + `site/`**. Do not treat glibc host binaries as the production artifact.

## Env

Copy `.env.example` → `.env`. Never commit `.env`.

Local Postgres: set `DATABASE_URL` for `cargo leptos watch` **and** soft-gated DB tests (same pool config story). Apply `sql/auth.sql` for admin sessions. `sql/posts.sql` is legacy.

## Notes

- `lib-profile-release = "wasm-release"` keeps client WASM small
- Prefer `rustls`; avoid deps that break zig/musl linking
- When islands land, update lib features here and in architecture.md
