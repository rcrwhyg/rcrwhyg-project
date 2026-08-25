# Build: static musl server (cargo-zigbuild)

Production goal: **SSR app** + **fully static Linux musl binary**.

## Prerequisites

```bash
# Zig (used as linker/cc by cargo-zigbuild)
# e.g. brew install zig   or   pip3 install ziglang

cargo install cargo-zigbuild --locked
rustup target add x86_64-unknown-linux-musl
# if VPS is ARM:
# rustup target add aarch64-unknown-linux-musl
```

## 1) Site assets (CSS / WASM islands)

```bash
cargo leptos build --release
```

Output: `target/site/` (and server binary under cargo-leptos’s usual path for local glibc/host builds).

## 2) Static server binary (musl)

From the project root, build the bin with SSR features for musl:

```bash
cargo zigbuild --release \
  --target x86_64-unknown-linux-musl \
  --features ssr \
  --bin rcrwhyg-server
```

Binary path (typical):

`target/x86_64-unknown-linux-musl/release/rcrwhyg-server`

Verify static linkage on Linux:

```bash
file target/x86_64-unknown-linux-musl/release/rcrwhyg-server
# expect: statically linked
```

## 3) Deploy layout

```text
/opt/rcrwhyg/
  rcrwhyg-server          # musl static binary
  site/                   # copy of target/site
```

Env (example):

```bash
export LEPTOS_SITE_ROOT="site"
export LEPTOS_SITE_PKG_DIR="pkg"
export LEPTOS_OUTPUT_NAME="rcrwhyg-server"
export LEPTOS_SITE_ADDR="0.0.0.0:3000"
# DATABASE_URL=...
./rcrwhyg-server
```

## Notes

- Prefer `rustls` stacks (already used by sqlx feature set) for musl friendliness.
- If a crate fails to link under zig/musl, replace it or vendor a pure-Rust alternative before adding glibc-only deps.
- Dev loop stays `cargo leptos watch` on the host; zigbuild is for **release/cross** artifacts.

## 4) Deploy via GitHub Actions

The build steps above are the same; the CD pipeline at
[`.github/workflows/cd.yml`](../.github/workflows/cd.yml) automates them.
A `git tag vX.Y.Z && git push origin vX.Y.Z` is all the user types —
the workflow builds, ships the binary + `site/` to the Aliyun ECS
instance, atomically swaps, restarts the systemd unit, and smoke-tests.

For the **one-time VPS bootstrap** (user creation, sudoers drop-in,
Caddy install, key install, .env), see
[`docs/deploy-vps.md`](deploy-vps.md). The atomic-swap script is
[`deploy/remote.sh`](../deploy/remote.sh). Rollback is a manual re-tag.
