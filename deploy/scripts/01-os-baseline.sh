#!/usr/bin/env bash
# 01-os-baseline.sh — 重装后的 OS 基线（Phase A）
# 以 root 运行： bash /root/bootstrap/01-os-baseline.sh
#
# 范围（按运维决策裁剪于 2026-08）：
#   只做 Swap 4G + swappiness=10（2C2G 防 OOM 的关键，见文章 02）+ 只读环境快照。
#   以下事项默认已满足/由运维手动处理，脚本不再触碰：
#     - apt 源：阿里 Ubuntu 镜像出厂即阿里云源，无需改写
#     - 时区/NTP：新实例默认 Asia/Shanghai（CST）且已同步
#     - 防火墙：阿里云控制台防火墙已收紧（22/80/443 only），服务器内不再加 ufw
#     - SSH 登录策略：保留密码登录（运维通过 Termius 使用）；部署账号 rcrwhyg 走密钥，不受影响
#
# 如需把本脚本之外的加固找回来，参见 docs/deploy-vps.md 对应章节（手动执行）。
set -euo pipefail

. /etc/os-release
echo "==> Ubuntu ${VERSION} (${VERSION_CODENAME})"

# ---------- 1. 只读环境快照（确认状态，不做任何修改） ----------
echo "--- 时区 ---"
timedatectl status | grep -E 'Time zone|synchronized' || true
echo "--- 内存 ---"
free -h
echo "--- 磁盘 ---"
df -h / | tail -n +1
echo "--- 当前 Swap 状态 ---"
swapon --show || echo '(无 swap)'

# ---------- 2. Swap 4G + swappiness=10（幂等） ----------
if ! swapon --show | grep -q '/swapfile'; then
  echo "==> 创建 4G swapfile..."
  fallocate -l 4G /swapfile 2>/dev/null || dd if=/dev/zero of=/swapfile bs=1M count=4096
  chmod 600 /swapfile
  mkswap /swapfile
  swapon /swapfile
else
  echo "==> swap 已存在，跳过创建。"
fi
grep -q '^/swapfile' /etc/fstab || echo '/swapfile none swap sw 0 0' >> /etc/fstab
sysctl -w vm.swappiness=10 >/dev/null
grep -q '^vm.swappiness=10' /etc/sysctl.conf || echo 'vm.swappiness=10' >> /etc/sysctl.conf

echo
echo "==> 完成。验证："
echo "    swapon --show"
echo "    sysctl vm.swappiness"
echo
echo "==> 按运维决策，以下请手动处理（本脚本不再执行）："
echo "    sudo apt update && sudo apt full-upgrade -y"
echo "    安装后续阶段所需基础包： sudo apt install -y curl ca-certificates gnupg apt-transport-https lsb-release rsync"