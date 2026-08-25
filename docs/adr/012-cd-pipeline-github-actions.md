# ADR-012: CD pipeline via GitHub Actions (tag-triggered)

## Status

Accepted (2026-08-25).

## Context

The repo had a fully-green CI workflow (5 quality gates on every push
to `master` and every PR) but no CD pipeline. Releases were a manual
loop: build locally with `cargo zigbuild`, `rsync` to the VPS, restart
the systemd unit by hand. This is error-prone and gives no audit trail.

## Decision

Add a second workflow, `.github/workflows/cd.yml`, that:

- **Triggers:** `push` of any `v*` tag (release) **or** `workflow_dispatch`
  with a `reason` input (manual hotfix / wiring check).
- **Runs on:** `ubuntu-latest` runner, single job `deploy-production`,
  `environment: production` (GitHub manual approval gate).
- **Builds** with the same toolchain + cache combo as CI, plus
  `cargo install --locked cargo-leptos` and
  `cargo install --locked cargo-zigbuild`.
- **Deploys** by streaming the built binary and `target/site/` to the
  VPS via `appleboy/scp-action@v1` and `appleboy/ssh-action@v1`. The
  swap logic lives in a version-controlled `deploy/remote.sh` that is
  SCP'd alongside the binary, so deploys are atomic and reproducible.
- **Smoke tests** `/health` and `/` over SSH (the runner can't reach
  `127.0.0.1:3000` on the VPS).
- **Finalizes** by writing a TSV line to `/opt/rcrwhyg/var/deploy.log`
  (date, tag, sha, actor, reason) and pruning old `site.prev.*`
  directories (keep last 3).

## Alternatives considered

- **Push-to-master auto-deploy** — too aggressive for a personal site;
  the tag/manual split is the conventional single-author pattern.
- **Stream via `actions/upload-artifact` + a separate download job** —
  double-storage, more moving parts. Direct SCP is simpler when there
  is one destination.
- **Auto-rollback on smoke failure** — race window: the broken binary
  may have already served traffic. Single-author project benefits from
  a known-good manual re-tag, which is one command.
- **Depot / deploybot / rsync.net** — paid services; out of scope for
  a free-tier personal site.

## Consequences

- VPS is reachable on 22 from GitHub Actions runners (verified by the
  Appleboy action).
- A new SSH keypair is provisioned; the public half lives only in
  `/opt/rcrwhyg/.ssh/authorized_keys`, the private half only as
  GitHub Secret `VPS_SSH_KEY`.
- The deploy script is shipped **on every release**, so any change to
  the swap/smoke logic takes effect on the next deploy without manual
  VPS surgery.
- Rollback is **manual** by re-tagging. See `docs/deploy-vps.md` §10.
