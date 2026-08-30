#!/usr/bin/env bash
# Post-build gate: release CSS must retain layout utilities (Tailwind purge regression).
set -euo pipefail

CSS="${1:-target/site/pkg/rcrwhyg-server.css}"
REQUIRED=(gap-4 px-4 max-w-4xl my-8 text-sm "sm:flex" space-y-5)

if [[ ! -f "$CSS" ]]; then
  echo "[FAIL] CSS not found: $CSS (run cargo leptos build --release first)" >&2
  exit 1
fi

missing=()
for token in "${REQUIRED[@]}"; do
  escaped="${token//:/\\:}"
  if grep -qF "$token" "$CSS" \
    || grep -qF "$escaped" "$CSS" \
    || grep -qE "\\.${escaped//./\\.}\\{" "$CSS"; then
    continue
  fi
  missing+=("$token")
done

if ((${#missing[@]})); then
  echo "[FAIL] $CSS missing Tailwind utilities (purge too aggressive):" >&2
  printf '  - %s\n' "${missing[@]}" >&2
  exit 1
fi

echo "[OK] site CSS contains required layout utilities ($(wc -c <"$CSS" | tr -d ' ') bytes)"
