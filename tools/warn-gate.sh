#!/usr/bin/env bash
# 构建 warning 门禁：扫描构建/测试日志，出现「未白名单」的 warning 即判红。
# 唯一白名单：proc-macro-error2 2.0.1 的未来不兼容提示。
#   该包是 leptos_macro 的依赖，截至 2026-08 上游无修复、cargo update 无新版本，
#   属于"我们改不了"的第三方告警；其余任何 warning 一律视为回归。
# 用法：bash tools/warn-gate.sh <日志文件>
set -euo pipefail

LOG="${1:?usage: warn-gate.sh <logfile>}"
[ -f "$LOG" ] || { echo "[warn-gate] 日志文件不存在: $LOG"; exit 2; }

# 注意：这里只盯 "warning"，不去拦 "note:" 等伴随行。
# 已知良性 warning 排除模式（其余任何 warning 均视为回归）：
#  - proc-macro-error2 2.0.1 未来不兼容（leptos_macro 上游依赖，未发修复）
#  - "linker stderr: ..."（zigld 的良性提示，如 "ignoring deprecated linker optimization setting"）
#  - "generated N warning"（Cargo 的汇总行；真正的具体 warning 行仍会被下面的过滤留下）
WARNINGS=$(perl -pe 's/\e\[[0-9;]*m//g' "$LOG" \
  | grep 'warning' \
  | grep -vF 'proc-macro-error2 v2.0.1' \
  | grep -vE 'warning: linker stderr:' \
  | grep -vE 'generated [0-9]+ warning' \
  | sed '/^$/d' || true)

if [ -n "$WARNINGS" ]; then
    echo "[warn-gate][FAIL] 构建/测试输出出现未白名单的 warning："
    printf '%s\n' "$WARNINGS" | head -40
    exit 1
fi
echo "[warn-gate] OK"