#!/usr/bin/env bash
# 04-app-user.sh — app 用户 + 布局 + .env + systemd（Phase D，CD 方案 A 布局）
# 以 root 运行。前提：
#   /root/bootstrap/rcrwhyg_deploy.pub 已被 scp 上来（CD 公钥）
#   /root/bootstrap/rcrwhyg.service     已被 scp 上来（仓库 deploy/systemd/rcrwhyg.service）
#   /root/bootstrap/schema/*.sql        已被 scp 上来（仓库 sql/posts.sql seed_posts.sql auth.sql）
#   /root/bootstrap/db_pw               仍存在（02 写入的强口令，本脚本用后删除）
set -euo pipefail

# ---------- 1. 系统用户与目录 ----------
useradd --system --create-home --home-dir /opt/rcrwhyg --shell /bin/bash rcrwhyg 2>/dev/null || true
install -d -o rcrwhyg -g rcrwhyg -m 0750 /opt/rcrwhyg/.ssh
install -d -o rcrwhyg -g rcrwhyg -m 0750 /opt/rcrwhyg/bin
install -d -o rcrwhyg -g rcrwhyg -m 0755 /opt/rcrwhyg/site
install -d -o rcrwhyg -g rcrwhyg -m 0750 /opt/rcrwhyg/var
install -d -o rcrwhyg -g rcrwhyg -m 0755 /opt/rcrwhyg/sql
id rcrwhyg

# ---------- 2. CD 公钥 -> authorized_keys ----------
test -f /root/bootstrap/rcrwhyg_deploy.pub \
  || { echo "==> 缺少 /root/bootstrap/rcrwhyg_deploy.pub，请先 scp。"; exit 1; }
install -m 0600 -o rcrwhyg -g rcrwhyg /root/bootstrap/rcrwhyg_deploy.pub /opt/rcrwhyg/.ssh/authorized_keys

# ---------- 3. sudoers drop-in（仅 CD 需要的 5 个命令） ----------
cat > /etc/sudoers.d/rcrwhyg <<'EOF'
rcrwhyg ALL=(root) NOPASSWD: /usr/bin/systemctl start rcrwhyg.service
rcrwhyg ALL=(root) NOPASSWD: /usr/bin/systemctl stop rcrwhyg.service
rcrwhyg ALL=(root) NOPASSWD: /usr/bin/systemctl restart rcrwhyg.service
rcrwhyg ALL=(root) NOPASSWD: /usr/bin/systemctl status rcrwhyg.service
rcrwhyg ALL=(root) NOPASSWD: /usr/bin/journalctl -u rcrwhyg.service *
EOF
chmod 0440 /etc/sudoers.d/rcrwhyg
visudo -c -f /etc/sudoers.d/rcrwhyg

# ---------- 4. .env（root:rcrwhyg 0640；口令运行时拼进 URL，不落入 git） ----------
DB_PASSFILE=/root/bootstrap/db_pw
[ -s "$DB_PASSFILE" ] || { echo "==> 缺少 $DB_PASSFILE（应已被 02 写入）。"; exit 1; }
DBPW="$(tr -d '\n' < "$DB_PASSFILE")"
DBCRED="rcrwhyg_user:${DBPW}"
install -o root -g rcrwhyg -m 0640 /dev/null /opt/rcrwhyg/.env
{
  echo '# /opt/rcrwhyg/.env - production only. root:rcrwhyg 0640. Never commit.'
  echo "DATABASE_URL=postgres://${DBCRED}@127.0.0.1:5432/rcrwhyg"
  echo 'COOKIE_SECURE=true'
  echo 'SESSION_TTL_HOURS=72'
  echo 'RATE_LIMIT_PUBLIC_PER_MIN=180'
  echo 'RATE_LIMIT_AUTH_PER_MIN=8'
  echo 'LEPTOS_SITE_ROOT=site'
  echo 'LEPTOS_SITE_PKG_DIR=pkg'
  echo 'LEPTOS_OUTPUT_NAME=rcrwhyg-server'
  echo 'LEPTOS_SITE_ADDR=127.0.0.1:3000'
  echo 'LEPTOS_ENV=PROD'
} >> /opt/rcrwhyg/.env
sudo -u rcrwhyg cat /opt/rcrwhyg/.env >/dev/null && echo '==> rcrwhyg 可读 .env OK'

# ---------- 5. schema 应用到新库（posts + seed + auth；属主 rcrwhyg_user） ----------
test -d /root/bootstrap/schema || { echo "==> 缺少 /root/bootstrap/schema/，请先 scp sql/*.sql。"; exit 1; }
install -m 0644 -o rcrwhyg -g rcrwhyg /root/bootstrap/schema/*.sql /opt/rcrwhyg/sql/
sudo -u rcrwhyg bash -c 'set -a; . /opt/rcrwhyg/.env; set +a
  /usr/lib/postgresql/18/bin/psql "$DATABASE_URL" \
    -v ON_ERROR_STOP=1 \
    -f /opt/rcrwhyg/sql/posts.sql \
    -f /opt/rcrwhyg/sql/seed_posts.sql \
    -f /opt/rcrwhyg/sql/auth.sql'

# ---------- 6. systemd unit（只 enable 不 start：二进制由首个 CD deploy 交付） ----------
test -f /root/bootstrap/rcrwhyg.service \
  || { echo "==> 缺少 /root/bootstrap/rcrwhyg.service，请先 scp。"; exit 1; }
install -o root -g root -m 0644 /root/bootstrap/rcrwhyg.service /etc/systemd/system/rcrwhyg.service
systemctl daemon-reload
systemctl enable rcrwhyg.service

# ---------- 7. 清理口令临时文件并验证 ----------
rm -f "$DB_PASSFILE"
echo "==> Phase D 完成。本地验证："
echo "    ssh rcrwhyg-prod 'whoami'"
echo "    ssh rcrwhyg-prod 'sudo -n systemctl status rcrwhyg.service || true'   # 未启动，正常"