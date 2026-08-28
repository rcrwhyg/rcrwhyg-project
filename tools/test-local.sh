#!/usr/bin/env bash
# 本地完整质量门禁（与 hooks/pre-push、远程 CI 同一套检查）
# 用法：./tools/test-local.sh

set -e

echo "== [1/5] cargo fmt --all --check =="
cargo fmt --all --check

echo "== [2/5] cargo clippy --features ssr --all-targets -- -D warnings =="
cargo clippy --features ssr --all-targets -- -D warnings

echo "== [3/5] cargo test --features ssr + warning 门禁 =="
set -o pipefail
cargo test --features ssr 2>&1 | tee /tmp/rct-test.log
bash "$(dirname "$0")/warn-gate.sh" /tmp/rct-test.log
set +o pipefail

echo "== [4/5] cargo check --lib --features hydrate --target wasm32-unknown-unknown + warning 门禁 =="
if rustup target list --installed 2>/dev/null | grep -q wasm32-unknown-unknown; then
    set -o pipefail
    cargo check --lib --features hydrate --target wasm32-unknown-unknown 2>&1 | tee /tmp/rct-wasm.log
    bash "$(dirname "$0")/warn-gate.sh" /tmp/rct-wasm.log
    set +o pipefail
else
    echo "[SKIP] 未安装 wasm32 target：rustup target add wasm32-unknown-unknown"
fi

echo "== [5/5] tools/check-articles.sh =="
./tools/check-articles.sh

echo ""
echo "本地门禁全部通过"
