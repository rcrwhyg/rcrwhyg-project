#!/usr/bin/env bash
# 02-pgdg-postgres.sh — 装 PostgreSQL 18 + 调优 + 建角色/库（Phase B）
# 以 root 运行： bash /root/bootstrap/02-pgdg-postgres.sh
# 口令：读取 /root/bootstrap/db_pw（本地生成，0600），不打印。
set -euo pipefail
. /etc/os-release

# 1) PGDG apt 源（阿里云 mirror）-> 安装 PG18（主集群 postgresql@18-main）
install -d -m 0755 /usr/share/postgresql-common/pgdg
curl -fsSLo /usr/share/postgresql-common/pgdg/apt.postgresql.org.asc \
  https://mirrors.aliyun.com/postgresql/repos/apt/ACCC4CF8.asc
echo "deb [signed-by=/usr/share/postgresql-common/pgdg/apt.postgresql.org.asc] \
https://mirrors.aliyun.com/postgresql/repos/apt/ ${VERSION_CODENAME}-pgdg main" \
  > /etc/apt/sources.list.d/pgdg.list
apt-get update
DEBIAN_FRONTEND=noninteractive apt-get install -y postgresql-18
systemctl enable --now postgresql@18-main

# 2) TCP(127.0.0.1/::1) 登录统一 scram（PGDG 默认即 scram，此行兜底；local peer 不变）
sed -E -i 's@^(host\s+all\s+all\s+(127\.0\.0\.1/32|::1/128)\s+).*@\1scram-sha-256@' \
  /etc/postgresql/18/main/pg_hba.conf || true

# 3) 文章02的 8 个调优参数
sudo -u postgres psql <<'SQL'
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
systemctl restart postgresql@18-main

# 4) 建角色 + 库（应用账号 rcrwhyg_user / 库 rcrwhyg）
DBPW="$(tr -d '\n' < /root/bootstrap/db_pw)"
sudo -u postgres psql -v ON_ERROR_STOP=1 \
  -c "CREATE ROLE rcrwhyg_user LOGIN PASSWORD '${DBPW}';" \
  -c "CREATE DATABASE rcrwhyg OWNER rcrwhyg_user ENCODING 'UTF8' TEMPLATE template0;"

# 5) 验证：应用账号能通过 TCP 登录
PGPASSWORD="$DBPW" /usr/lib/postgresql/18/bin/psql \
  "postgres://rcrwhyg_user@127.0.0.1:5432/rcrwhyg" -Atc 'SELECT version();'
echo "==> Phase B 完成"