# Deploy: VPS bootstrap (Aliyun ECS, x86_64)

This runbook is the **one-time setup** for the Aliyun ECS instance that
hosts the rcrwhyg-server production binary. After it completes, every
release is just `git tag vX.Y.Z && git push origin vX.Y.Z`.

> **推荐执行方式**：直接用仓库脚本 `deploy/scripts/01-os-baseline.sh` →
> `02-pgdg-postgres.sh` → `03-caddy.sh` → `04-app-user.sh`（本手册命令即
> 各脚本内容，二者等价）。**PostgreSQL 走 PGDG 18**（Ubuntu/Debian 布局：
> 配置 `/etc/postgresql/18/main/postgresql.conf`、数据
> `/var/lib/postgresql/18/main`、服务 `postgresql@18-main`；`/var/lib/pgsql/<ver>/data`
> 是 RPM 发行版路径，不适用 Ubuntu）。`create-admin` 不经 CD 部署，通过
> 本地 SSH 隧道连远端 5432 运行一次即可（见 §11）。

## 0. Assumptions

- **OS:** Ubuntu 22.04 LTS or Debian 12 (both validated; pick Ubuntu for
  the Aliyun Caddy mirror path documented here).
- **Network:** outbound 443 (LE / crates.io / Zig download); inbound
  22, 80, 443 from the public internet. **3000 stays on 127.0.0.1.**
- **CPU:** x86_64.
- **Local box:** has `gh` and `ssh-keygen`; the deploy SSH key is
  generated **locally** and the public half is pasted into the VPS.

## 1. System user

```bash
sudo useradd --system --create-home --home-dir /opt/rcrwhyg --shell /bin/bash rcrwhyg
sudo install -d -o rcrwhyg -g rcrwhyg -m 0750 /opt/rcrwhyg/.ssh
sudo install -d -o rcrwhyg -g rcrwhyg -m 0750 /opt/rcrwhyg/bin
sudo install -d -o rcrwhyg -g rcrwhyg -m 0755 /opt/rcrwhyg/site
sudo install -d -o rcrwhyg -g rcrwhyg -m 0750 /opt/rcrwhyg/var
id rcrwhyg   # verify
```

## 2. Sudoers drop-in (narrow)

`rcrwhyg` is no-password, no-sudo by default. We give it a tightly
scoped sudoers drop-in so the CD pipeline can manage its own service
without `root` being on the wire.

```bash
sudo tee /etc/sudoers.d/rcrwhyg >/dev/null <<'EOF'
rcrwhyg ALL=(root) NOPASSWD: /usr/bin/systemctl start rcrwhyg.service
rcrwhyg ALL=(root) NOPASSWD: /usr/bin/systemctl stop rcrwhyg.service
rcrwhyg ALL=(root) NOPASSWD: /usr/bin/systemctl restart rcrwhyg.service
rcrwhyg ALL=(root) NOPASSWD: /usr/bin/systemctl status rcrwhyg.service
rcrwhyg ALL=(root) NOPASSWD: /usr/bin/journalctl -u rcrwhyg.service *
EOF
sudo chmod 0440 /etc/sudoers.d/rcrwhyg
sudo visudo -c -f /etc/sudoers.d/rcrwhyg   # must print: parsed OK
```

## 3. SSH keypair for deploy

**On your local box** (not the VPS):

```bash
ssh-keygen -t ed25519 -N '' -C 'rcrwhyg-deploy-key' -f ~/.ssh/rcrwhyg_deploy
cat ~/.ssh/rcrwhyg_deploy.pub   # copy the printed line
```

**On the VPS**, install the public half (one-time):

```bash
echo '<paste the .pub line>' | sudo tee -a /opt/rcrwhyg/.ssh/authorized_keys
sudo chown rcrwhyg:rcrwhyg /opt/rcrwhyg/.ssh/authorized_keys
sudo chmod 0600 /opt/rcrwhyg/.ssh/authorized_keys
```

**Verify from local:**

```bash
ssh -i ~/.ssh/rcrwhyg_deploy -p 22 rcrwhyg@<VPS_HOST> \
  'whoami && sudo -n systemctl status rcrwhyg.service || true'
# Expect: rcrwhyg, then "Unit rcrwhyg.service could not be found." (we
# haven't enabled it yet — that's fine).
```

The private half goes into GitHub as Secret `VPS_SSH_KEY` (see step 8).

## 4. Install Caddy

Official apt repo (covers Ubuntu 22.04 + Debian 12). If the Aliyun
mirror is blocked in your region, fall back to direct download from
`https://github.com/caddyserver/caddy/releases`.

```bash
sudo apt install -y debian-keyring debian-archive-keyring apt-transport-https curl
curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/gpg.key' \
  | sudo gpg --dearmor -o /usr/share/keyrings/caddy-stable-archive-keyring.gpg
curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/deb/debian/dists/any-version/main/binary-amd64/Packages' > /dev/null
echo "deb [signed-by=/usr/share/keyrings/caddy-stable-archive-keyring.gpg] https://dl.cloudsmith.io/public/caddy/stable/deb/debian any-version main" \
  | sudo tee /etc/apt/sources.list.d/caddy-stable.list
sudo apt update
sudo apt install -y caddy
caddy version   # >= 2.7
```

## 5. Drop in Caddyfile

```bash
sudo cp deploy/caddy/Caddyfile /etc/caddy/Caddyfile
sudo sed -i 's/<DOMAIN>/your.actual.domain.cn/g' /etc/caddy/Caddyfile
sudo caddy validate --config /etc/caddy/Caddyfile
sudo systemctl reload caddy
systemctl status caddy --no-pager
```

**Aliyun CAS alternative** (faster China-mainland TLS, no LE round-trip):
pre-issue a cert via CAS, drop the cert+key at
`/etc/caddy/certs/<DOMAIN>.crt` and `/etc/caddy/certs/<DOMAIN>.key`, then
add this line **inside** the site block (after `encode`):

```caddy
tls /etc/caddy/certs/<DOMAIN>.crt /etc/caddy/certs/<DOMAIN>.key
```

Caddy will then skip the LE issuance and serve the CAS cert.

## 6. Environment file

The Rust binary reads its env from `/opt/rcrwhyg/.env` at boot. Owned
by `root:rcrwhyg`, mode `0640` — `rcrwhyg` can read but never edit.

```bash
sudo install -o root -g rcrwhyg -m 0640 /dev/null /opt/rcrwhyg/.env
sudo tee /opt/rcrwhyg/.env >/dev/null <<'EOF'
DATABASE_URL=<your Postgres connection string; see .env.example for the format>
COOKIE_SECURE=true
SESSION_TTL_HOURS=72
RATE_LIMIT_PUBLIC_PER_MIN=180
RATE_LIMIT_AUTH_PER_MIN=8
LEPTOS_SITE_ROOT=site
LEPTOS_SITE_PKG_DIR=pkg
LEPTOS_OUTPUT_NAME=rcrwhyg-server
LEPTOS_SITE_ADDR=127.0.0.1:3000
LEPTOS_ENV=PROD
EOF
sudo -u rcrwhyg cat /opt/rcrwhyg/.env >/dev/null && echo "rcrwhyg can read .env OK"
```

> ⚠️ **Do not reuse the local-dev `alon@123456` password from
> `.env.example` in production.** Provision a dedicated DB user and a
> strong password; ideally use a managed Postgres (RDS) with TLS.

## 7. systemd unit

```bash
sudo cp deploy/systemd/rcrwhyg.service /etc/systemd/system/rcrwhyg.service
sudo systemctl daemon-reload
sudo systemctl enable rcrwhyg.service
# Do NOT start yet — the binary is not there. The first CD run will start it.
```

## 8. GitHub repo configuration

In `https://github.com/rcrwhyg/rcrwhyg-project/settings`:

**Secrets (Settings → Secrets and variables → Actions):**

| Secret | Value |
|--------|-------|
| `VPS_SSH_KEY` | contents of local `~/.ssh/rcrwhyg_deploy` (the **private** key) |
| `VPS_HOST` | Aliyun ECS public IP or hostname |
| `VPS_USER` | `rcrwhyg` |
| `VPS_PORT` | `22` |

**Environment (Settings → Environments → `production`):**

- Required reviewers: add yourself (the only human who can approve)
- Deployment branches and tags: add tag pattern `v*`

## 9. First deploy

```bash
# Local box
git tag v0.1.0
git push origin v0.1.0

# Watch
gh run watch

# After green, on the VPS:
ssh -i ~/.ssh/rcrwhyg_deploy rcrwhyg@<VPS_HOST>
   systemctl status rcrwhyg --no-pager    # active (running)
   tail -n 1 /opt/rcrwhyg/var/deploy.log  # the latest deploy entry
   curl -fsS http://127.0.0.1:3000/health
   exit

# Public-facing
curl -fsSI https://<DOMAIN>/             # 200 with HSTS
```

## 10. Rollback (manual, by design)

The CD pipeline does **not** auto-rollback. To roll back:

```bash
git log --oneline v0.1.0   # find the previous good SHA
git tag -f v0.1.0 <PREVIOUS_GOOD_SHA>
git push --force origin v0.1.0
gh run watch   # the atomic swap will put the old commit's binary on disk
```

## 11. DB migrations

The CD pipeline does **not** run migrations. To apply schema changes:

1. `ssh` to the DB host (or `psql` from a jump box with TLS)
2. `psql "$DATABASE_URL" -f sql/posts.sql` (or whatever's relevant)
3. Tag and push as usual; the new binary picks up the new schema on next deploy

`DATABASE_URL` is **never** in CI. Only the binary on the VPS sees it.
