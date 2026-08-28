#!/usr/bin/env bash
# deploy/remote.sh — runs ON the VPS as the rcrwhyg user.
# Called by .github/workflows/cd.yml via appleboy/ssh-action.
set -euo pipefail

DEPLOY_ROOT="${DEPLOY_ROOT:-/opt/rcrwhyg}"
SITE_NAME="site"

cmd="${1:-}"; shift || true

case "$cmd" in
  swap)
    cd "$DEPLOY_ROOT"
    if [[ -f rcrwhyg-server.new ]]; then
      chmod 0755 rcrwhyg-server.new
      [[ -f rcrwhyg-server ]] && mv -f rcrwhyg-server rcrwhyg-server.prev
      mv -f rcrwhyg-server.new rcrwhyg-server
    fi
    if [[ -d ${SITE_NAME}.staging ]]; then
      [[ -d ${SITE_NAME} ]]     && mv -f "${SITE_NAME}"     "${SITE_NAME}.prev"
      mv -f "${SITE_NAME}.staging" "${SITE_NAME}"
    fi
    ;;
  smoke)
    for i in $(seq 1 30); do
      if curl -fsS -o /dev/null "http://127.0.0.1:3000/health" 2>/dev/null; then
        curl -fsS "http://127.0.0.1:3000/health"; echo
        curl -fsS -o /dev/null "http://127.0.0.1:3000/"
        echo "smoke OK"; exit 0
      fi
      sleep 1
    done
    echo "smoke FAILED: service did not come up within 30s" >&2
    sudo -n journalctl -u rcrwhyg.service -n 80 --no-pager >&2 || true
    exit 1
    ;;
  finalize)
    tag="${1:-unknown}"; sha="${2:-unknown}"; actor="${3:-unknown}"; reason="${4:-tag-push}"
    cd "$DEPLOY_ROOT"
    rm -f rcrwhyg-server.new
    rm -rf "${SITE_NAME}.staging"
    mkdir -p var
    printf '%s\t%s\t%s\t%s\t%s\n' \
      "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$tag" "$sha" "$actor" "$reason" \
      >> var/deploy.log
    ls -1dt "${SITE_NAME}.prev".* 2>/dev/null \
      | tail -n +4 \
      | xargs -r rm -rf \
      || true   # 首次部署没有 .prev，ls 无匹配退出 1；pipefail 下必须兜底
    ;;
  *)
    echo "usage: $0 {swap|smoke|finalize ...}" >&2
    exit 64
    ;;
esac
