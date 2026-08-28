# 如春日午后阳光

[![CI](https://github.com/rcrwhyg/rcrwhyg-project/actions/workflows/ci.yml/badge.svg)](https://github.com/rcrwhyg/rcrwhyg-project/actions/workflows/ci.yml)
[![CD](https://github.com/rcrwhyg/rcrwhyg-project/actions/workflows/cd.yml/badge.svg)](https://github.com/rcrwhyg/rcrwhyg-project/actions/workflows/cd.yml)

Personal site: Leptos 0.8 SSR-first + Axum (HTTP / planned WS+SSE) + Tailwind. Calm-tech UI (modern, minimal, dynamic) with dark/light themes and a dynamic mouse-follow backdrop. Production server: **static musl** via **cargo-zigbuild**.

## Docs

- [Architecture](docs/architecture.md)
- [Quality gates](docs/quality-gates.md)（代码 + 文章双重门禁）
- [Testing](docs/testing.md)
- [ADRs](docs/adr/)
- [musl / zigbuild release](docs/build-musl.md)
- Env template: [`.env.example`](.env.example)

## AI 协作与规范

- 总纲：[AGENT.md](AGENT.md)（权限边界、工作流、门禁纪律）
- 规则：[rules/](rules/)（git 工作流、代码质量、文章质量）
- 文章规范：[specs/article-template.md](specs/article-template.md)、个人网站首发文章目录 [articles/](articles/)（公众号【如春日午后阳光】为转载渠道）

## 质量门禁

```bash
./tools/install-hooks.sh   # 首次克隆后安装 git 钩子
./tools/test-local.sh      # 本地全量门禁：fmt + clippy(-D) + test + wasm + 文章检查
```

推送后远程 CI（GitHub Actions，见 `.github/workflows/ci.yml`）须全绿，见 [docs/quality-gates.md](docs/quality-gates.md)。

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

## Deploy

Production deploy is automated via GitHub Actions. Tag and push:

```bash
git tag v0.1.0
git push origin v0.1.0
```

The CD workflow builds the musl binary + site assets, streams them to the
Aliyun ECS instance, atomically swaps files, restarts the systemd unit,
and smoke-tests `/health`. Required: GitHub Secrets `VPS_HOST`,
`VPS_USER`, `VPS_PORT`, `VPS_SSH_KEY` and a `production` environment with
yourself as required reviewer.

- Workflow: [`.github/workflows/cd.yml`](.github/workflows/cd.yml)
- VPS bootstrap runbook: [`docs/deploy-vps.md`](docs/deploy-vps.md)
- Deploy assets: [`deploy/`](deploy/)

## Agent skills

- **Project**: `.cursor/skills/leptos-*` (incl. `leptos-ui-theme-chrome`, `leptos-ecosystem-patterns`)
- **Global**: `~/.cursor/skills/rust-orientation`, `rust-axum-tokio`, `rust-sqlx-postgres`

## License

See [LICENSE](LICENSE).
