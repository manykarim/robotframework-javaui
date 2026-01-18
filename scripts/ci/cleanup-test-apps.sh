#!/usr/bin/env bash
set -euo pipefail

patterns=(
  "swing-test-app-1.0.0.jar"
  "swt-test-app-1.0.0-all.jar"
  "rcp-mock-test-app-1.0.0-all.jar"
)

for pattern in "${patterns[@]}"; do
  pkill -f "${pattern}" || true
done

sleep 1
