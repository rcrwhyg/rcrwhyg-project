#!/usr/bin/env bash
# 03-caddy.sh — Caddy 安装 + 仓库加固 Caddyfile（Phase C）
# 以 root 运行。前提：/root/bootstrap/Caddyfile（仓库 deploy/caddy/Caddyfile）已被 scp 上来。
set -euo pipefail

# ---------- 1. 官方 apt 源（cloudsmith；同一仓库服务所有 Ubuntu 版本） ----------
DEBIAN_FRONTEND=noninteractive apt-get install -y debian-keyring debian-archive-keyring apt-transport-https curl
curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/gpg.key' \
  | gpg --dearmor -o /usr/share/keyrings/caddy-stable-archive-keyring.gpg
echo "deb [signed-by=/usr/share/keyrings/caddy-stable-archive-keyring.gpg] https://dl.cloudsmith.io/public/caddy/stable/deb/debian any-version main" \
  > /etc/apt/sources.list.d/caddy-stable.list
apt-get update
DEBIAN_FRONTEND=noninteractive apt-get install -y caddy
caddy version

# ---------- 2. 落地仓库 Caddyfile 并替换占位符 ----------
test -f /root/bootstrap/Caddyfile || { echo "==> 缺少 /root/bootstrap/Caddyfile，请先 scp 仓库的 deploy/caddy/Caddyfile。"; exit 1; }
install -o root -g root -m 0644 /root/bootstrap/Caddyfile /etc/caddy/Caddyfile
sed -i 's/<DOMAIN>/rcrwhyg.com, www.rcrwhyg.com/' /etc/caddy/Caddyfile
sed -i 's/ops@<DOMAIN>/rcrwhyg@sina.com/' /etc/caddy/Caddyfile
caddy validate --config /etc/caddy/Caddyfile
systemctl reload caddy
systemctl enable --now caddy
systemctl status caddy --no-pager | head -12

echo
echo "==> Phase C 完成。验证："
echo "    ss -tlnp | grep -E ':(80|443) '"
echo "    curl -fsSI https://rcrwhyg.com/   # 首个请求 Caddy 自动签 Let's Encrypt 证书"