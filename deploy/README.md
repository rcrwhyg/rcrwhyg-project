# deploy/ — production deployment assets

These files are the **source of truth** for the production environment on
the Aliyun ECS instance. Everything here is open-source and unencrypted.
Secrets never live in this directory.

| File | Where it lands on the VPS | Purpose |
|------|---------------------------|---------|
| `remote.sh` | `/opt/rcrwhyg/bin/remote.sh` | Atomic swap + smoke test + log finalization; called by `.github/workflows/cd.yml` |
| `systemd/rcrwhyg.service` | `/etc/systemd/system/rcrwhyg.service` | Service unit (runs as `rcrwhyg`, hardened) |
| `caddy/Caddyfile` | `/etc/caddy/Caddyfile` | Reverse proxy + auto-TLS (replace `<DOMAIN>` first) |

For the **one-time VPS bootstrap** (user creation, sudoers drop-in, key
install, Caddy install, .env, systemd enable), see
[`../docs/deploy-vps.md`](../docs/deploy-vps.md).

The CD workflow is [`../.github/workflows/cd.yml`](../.github/workflows/cd.yml).
