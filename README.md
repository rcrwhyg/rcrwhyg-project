# 如春日午后阳光

Personal site: Leptos 0.8 SSR-first + Axum (HTTP / planned WS+SSE) + Tailwind. Extreme cyberpunk UI with dark/light themes and a dynamic mouse-follow backdrop. Production server: **static musl** via **cargo-zigbuild**.

## Docs

- [Architecture](docs/architecture.md)
- [Testing](docs/testing.md)
- [ADRs](docs/adr/)
- [musl / zigbuild release](docs/build-musl.md)
- Env template: [`.env.example`](.env.example)

## Develop

```bash
cargo install cargo-leptos --locked   # once
cargo leptos watch
```

## Release

```bash
cargo leptos build --release
cargo zigbuild --release --target x86_64-unknown-linux-musl --features ssr --bin rcrwhyg-server
```

Deploy the musl binary + `target/site`. Details in [docs/build-musl.md](docs/build-musl.md).

## Agent skills

- **Project**: `.cursor/skills/leptos-*` (incl. `leptos-ui-theme-chrome`, `leptos-ecosystem-patterns`)
- **Global**: `~/.cursor/skills/rust-orientation`, `rust-axum-tokio`, `rust-sqlx-postgres`

## License

See [LICENSE](LICENSE).
