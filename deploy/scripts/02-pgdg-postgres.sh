#!/usr/bin/env bash
# 02-pgdg-postgres.sh — PostgreSQL 18 via PGDG（Phase B）
# 以 root 运行。Debian/Ubuntu 集群布局（PGDG on Ubuntu 即此）：
#   配置 /etc/postgresql/18/main/postgresql.conf，数据 /var/lib/postgresql/18/main，
#   服务 postgresql@18-main（元服务 postgresql.service）。
# 注意：/var/lib/pgsql/18/data 是 RPM 发行版（RHEL/Fedora）路径，不适用于 Ubuntu。
# 数据库口令：本地 openssl rand -hex 24 生成后写入 /root/bootstrap/db_pw（chmod 600），本脚本读取；
#              Phase D 写 .env 后再删除。
set -euo pipefail

. /etc/os-release
CODENAME="${VERSION_CODENAME}"          # resolute | noble

# ---------- 1. PGDG apt 源（阿里云 mirror，已核实含两种 suite + ACCC4CF8.asc） ----------
install -d -m 0755 /usr/share/postgresql-common/pgdg
curl -fsSLo /usr/share/postgresql-common/pgdg/apt.postgresql.org.asc \
  https://mirrors.aliyun.com/postgresql/repos/apt/ACCC4CF8.asc
# 若阿里云 mirror 不可达，改用官方源：
#   curl -fsSLo /usr/share/postgresql-common/pgdg/apt.postgresql.org.asc \
#     https://www.postgresql.org/media/keys/ACCC4CF8.asc
#   并把下两行中的 mirrors.aliyun.com/postgresql/repos/apt 换成 apt.postgresql.org
echo "deb [signed-by=/usr/share/postgresql-common/pgdg/apt.postgresql.org.asc] \
https://mirrors.aliyun.com/postgresql/repos/apt/ ${CODENAME}-pgdg main" \
  > /etc/apt/sources.list.d/pgdg.list
apt-get update
apt-cache policy postgresql-18 | head -5

# ---------- 2. 安装 PG 18 并启动主集群 ----------
DEBIAN_FRONTEND=noninteractive apt-get install -y postgresql-18
systemctl enable --now postgresql@18-main
systemctl status postgresql@18-main --no-pager | head -12
PGDATA=/var/lib/postgresql/18/main
test -f "$PGDATA/PG_VERSION" || { echo "==> 主集群未自动初始化（意外）；请检查后继续。"; exit 1; }

# ---------- 3. pg_hba：local peer 保持，TCP 127.0.0.1/::1 改 scram-sha-256 ----------
cp -n "$PGDATA/pg_hba.conf" "$PGDATA/pg_hba.conf.bak" || true
sed -E -i 's@^(host\s+all\s+all\s+127\.0\.0\.1/32\s+).*@\1scram-sha-256@' /etc/postgresql/18/main/pg_hba.conf
sed -E -i 's@^(host\s+all\s+all\s+::1/128\s+).*@\1scram-sha-256@'    /etc/postgresql/18/main/pg_hba.conf
grep -E '^(local|host)' /etc/postgresql/18/main/pg_hba.conf
systemctl reload postgresql@18-main

# ---------- 4. 建角色 + 库（口令来自 /root/bootstrap/db_pw，不打印） ----------
DB_PASSFILE=/root/bootstrap/db_pw
[ -s "$DB_PASSFILE" ] || { echo "==> 缺少 $DB_PASSFILE：请把本地生成的强口令写入（chmod 600）。"; exit 1; }
chmod 600 "$DB_PASSFILE"
DBPW="$(tr -d '\n' < "$DB_PASSFILE")"
sudo -u postgres psql -v ON_ERROR_STOP=1 \
  -c "CREATE ROLE rcrwhyg_user LOGIN PASSWORD '${DBPW}';" \
  -c "CREATE DATABASE rcrwhyg OWNER rcrwhyg_user ENCODING 'UTF8' TEMPLATE template0;"

# ---------- 5. 文章 02 的 8 个调优参数（ALTER SYSTEM，写入 postgresql.auto.conf） ----------
sudo -u postgres psql -v ON_ERROR_STOP=1 <<'SQL'
ALTER SYSTEM SET shared_buffers = '256MB';
ALTER SYSTEM SET max_connections = 30;
ALTER SYSTEM SET work_mem = '4MB';
ALTER SYSTEM SET maintenance_work_mem = '64MB';
ALTER SYSTEM SET effective_cache_size = '1GB';
ALTER SYSTEM SET random_page_cost = 1.1;
ALTER SYSTEM SET wal_buffers = '8MB';
ALTER SYSTEM SET min_wal_size = '80MB';
ALTER SYSTEM SET max_wal_size = '500MB';
SQL
# shared_buffers / max_connections / wal_buffers 需重启生效
systemctl restart postgresql@18-main

# ---------- 6. 验证 ----------
sudo -u postgres psql -Atc "SHOW shared_buffers; SHOW max_connections; SHOW work_mem; SHOW maintenance_work_mem; SHOW effective_cache_size; SHOW random_page_cost; SHOW wal_buffers; SHOW min_wal_size; SHOW max_wal_size;"
PGPASSWORD="$DBPW" /usr/lib/postgresql/18/bin/psql \
  "postgres://rcrwhyg_user@127.0.0.1:5432/rcrwhyg" -Atc 'SELECT version();'

echo
echo "==> Phase B 完成。db_pw 保留到 Phase D（写入 .env）之后再删除。"