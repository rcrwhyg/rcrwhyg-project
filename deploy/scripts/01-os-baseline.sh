#!/usr/bin/env bash
# 01-os-baseline.sh — 重装后的 OS 基线（Phase A）
# 以 root 运行： bash /root/bootstrap/01-os-baseline.sh
# 覆盖：apt 阿里云源 / 时区+ntp / Swap 4G / swappiness=10 / 基础包 / admin 公钥 / ufw / 关闭口令登录
# 幂等，可分段重复跑。任何一步失败立即退出（set -euo pipefail）。
set -euo pipefail

. /etc/os-release
CODENAME="${VERSION_CODENAME}"          # resolute (26.04) | noble (24.04)
echo "==> Ubuntu ${VERSION} (${CODENAME})"

# ---------- 1. apt 源 -> 阿里云（内网 mirror，ECS 免公网流量） ----------
cat > /etc/apt/sources.list <<EOF
deb http://mirrors.cloud.aliyuncs.com/ubuntu/ ${CODENAME} main restricted universe multiverse
deb http://mirrors.cloud.aliyuncs.com/ubuntu/ ${CODENAME}-updates main restricted universe multiverse
deb http://mirrors.cloud.aliyuncs.com/ubuntu/ ${CODENAME}-backports main restricted universe multiverse
deb http://mirrors.cloud.aliyuncs.com/ubuntu/ ${CODENAME}-security main restricted universe multiverse
EOF
chmod 0644 /etc/apt/sources.list
# 若内网 mirror 不可达（apt update 报连接失败），改用公网阿里云镜像：
#   sed -i 's#mirrors.cloud.aliyuncs.com#mirrors.aliyun.com#' /etc/apt/sources.list
apt-get update
DEBIAN_FRONTEND=noninteractive apt-get full-upgrade -y
apt-get autoremove -y

# ---------- 2. 时区 + NTP（必须先于 Postgres 初始化） ----------
timedatectl set-timezone Asia/Shanghai
timedatectl set-ntp true
timedatectl status

# ---------- 3. Swap 4G + swappiness=10（与文章 02 一致） ----------
if ! swapon --show | grep -q '/swapfile'; then
  fallocate -l 4G /swapfile 2>/dev/null || dd if=/dev/zero of=/swapfile bs=1M count=4096
  chmod 600 /swapfile
  mkswap /swapfile
  swapon /swapfile
fi
grep -q '^/swapfile' /etc/fstab || echo '/swapfile none swap sw 0 0' >> /etc/fstab
sysctl -w vm.swappiness=10 >/dev/null
grep -q '^vm.swappiness=10' /etc/sysctl.conf || echo 'vm.swappiness=10' >> /etc/sysctl.conf
swapon --show
sysctl vm.swappiness

# ---------- 4. 基础包 ----------
DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
  curl wget ca-certificates gnupg apt-transport-https lsb-release ufw rsync htop

# ---------- 5. SSH：先装 admin 公钥（密钥在前，口令开关在后，顺序错了会锁死自己） ----------
# 前提： /root/bootstrap/rcrwhyg_admin.pub 已被本地 scp 上来
if [ -f /root/bootstrap/rcrwhyg_admin.pub ]; then
  install -d -m 0700 -o root -g root /root/.ssh
  install -m 0600 -o root -g root /root/bootstrap/rcrwhyg_admin.pub /root/.ssh/authorized_keys
  echo
  echo "==> admin 公钥已安装。请【另开一个终端】验证："
  echo "    ssh -i ~/.ssh/rcrwhyg_admin rcrwhyg-admin 'echo admin-key-ok'"
  echo "    确认返回 admin-key-ok 后，按回车继续；否则 Ctrl-C 中止排查。"
  read -r -p '==> 输入回车继续：' _
else
  echo "==> 未找到 /root/bootstrap/rcrwhyg_admin.pub；跳过密钥安装。"
  echo "    若你不打算用密钥（不推荐），可直接 Ctrl-C 后重跑第 5 步之后的部分。"
fi

# ---------- 6. ufw：先放行 22/80/443 再 enable ----------
# 只有在已确认能密钥登录时才继续（否则你随时可能断线）。
[ -s /root/.ssh/authorized_keys ] || { echo "==> authorized_keys 为空，拒绝继续（防锁死）。"; exit 1; }
ufw allow 22/tcp comment 'SSH'
ufw allow 80/tcp comment 'HTTP'
ufw allow 443/tcp comment 'HTTPS'
ufw --force enable
ufw status verbose

# ---------- 7. 关闭 SSH 口令登录（authorized_keys 非空守卫已在上一步） ----------
# 用 drop-in 覆盖（优先级高于主配置/cloud-init）
install -m 0644 -o root -g root /dev/null /etc/ssh/sshd_config.d/60-hardening.conf
grep -q '^PasswordAuthentication no' /etc/ssh/sshd_config.d/60-hardening.conf \
  || echo 'PasswordAuthentication no' >> /etc/ssh/sshd_config.d/60-hardening.conf
systemctl restart ssh

echo
echo "==> Phase A 完成。验证："
echo "    ssh -i ~/.ssh/rcrwhyg_admin rcrwhyg-admin 'hostname; whoami'"