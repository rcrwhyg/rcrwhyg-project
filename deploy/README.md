# deploy/ — production deployment assets

These files are the **source of truth** for the production environment on
the Aliyun ECS instance. Everything here is open-source and unencrypted.
Secrets (DB password, SSH keys) never live in this directory — they are
created on the VPS / in GitHub Secrets only.

| File | Where it lands on the VPS | Purpose |
|------|---------------------------|---------|
| `remote.sh` | `/opt/rcrwhyg/bin/remote.sh` | Atomic swap + smoke test + log finalization; called by `.github/workflows/cd.yml` |
| `scripts/01-os-baseline.sh` | run on VPS (root) | OS baseline: apt mirror, timezone, swap, SSH key + ufw hardening |
| `scripts/02-pgdg-postgres.sh` | run on VPS (root) | PostgreSQL 18 via PGDG + article-02 tuning + app role/db |
| `scripts/03-caddy.sh` | run on VPS (root) | Caddy install + repo Caddyfile deploy |
| `scripts/04-app-user.sh` | run on VPS (root) | `rcrwhyg` user, `/opt/rcrwhyg` layout, sudoers, .env, systemd unit |
| `systemd/rcrwhyg.service` | `/etc/systemd/system/rcrwhyg.service` | Service unit (runs as `rcrwhyg`, hardened) |
| `caddy/Caddyfile` | `/etc/caddy/Caddyfile` | Reverse proxy + auto-TLS (`<DOMAIN>`→real domain on deploy) |

For the **one-time VPS bootstrap** run the scripts in order `01 → 04`
(see [`docs/deploy-vps.md`](../docs/deploy-vps.md)). PostgreSQL is
installed via PGDG 18: config at `/etc/postgresql/18/main/postgresql.conf`,
data at `/var/lib/postgresql/18/main`, unit `postgresql@18-main`
(Debian/Ubuntu layout — the `/var/lib/pgsql/<ver>/data` path is the
RPM-distro variant, NOT Ubuntu).

The CD workflow is [`../.github/workflows/cd.yml`](../.github/workflows/cd.yml).
