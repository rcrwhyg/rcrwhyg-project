#!/usr/bin/env bash
# 构建 warning 门禁：扫描构建/测试日志，出现「未白名单」的 warning 即判红。
# 唯一白名单：proc-macro-error2 2.0.1 的未来不兼容提示。
#   该包是 leptos_macro 的依赖，截至 2026-08 上游无修复、cargo update 无新版本，
#   属于"我们改不了"的第三方告警；其余任何 warning 一律视为回归。
# 用法：bash tools/warn-gate.sh <日志文件>
set -euo pipefail

LOG="${1:?usage: warn-gate.sh <logfile>}"
[ -f "$LOG" ] || { echo "[warn-gate] 日志文件不存在: $LOG"; exit 2; }

ALLOW='warning: the following packages contain code that will be rejected by a future version of Rust: proc-macro-error2 v2.0.1'

# 注意：这里只盯 "warning"，不去拦 "note:" 等伴随行。
# Cargo 在 CARGO_TERM_COLOR=always 时会给输出加 ANSI 色码，先剥掉再比对/WARNING。
WARNINGS=$(perl -pe 's/\e\[[0-9;]*m//g' "$LOG" \
  | grep 'warning' \
  | grep -Fxv "$ALLOW" \
  | sed '/^$/d' || true)

if [ -n "$WARNINGS" ]; then
    echo "[warn-gate][FAIL] 构建/测试输出出现未白名单的 warning："
    printf '%s\n' "$WARNINGS" | head -40
    exit 1
fi
echo "[warn-gate] OK"