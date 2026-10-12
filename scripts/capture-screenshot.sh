#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-or-later
# Run this script under Xvfb to capture the actual LIRUVA application window.
set -euo pipefail

OUTPUT="${1:?Provide an output PNG path}"
APP_LOG="${RUNNER_TEMP:-/tmp}/liruva-screenshot-app.log"
mkdir -p "$(dirname "$OUTPUT")"

export LIBGL_ALWAYS_SOFTWARE=1
export WGPU_BACKEND=gl
./target/debug/liruva >"$APP_LOG" 2>&1 &
APP_PID=$!
trap 'kill "$APP_PID" 2>/dev/null || true' EXIT

WINDOW=""
for _ in $(seq 1 90); do
    if ! kill -0 "$APP_PID" 2>/dev/null; then
        echo "LIRUVA closed before its window became available."
        cat "$APP_LOG"
        exit 1
    fi

    WINDOW="$(xdotool search --onlyvisible --name '^LIRUVA' 2>/dev/null | head -n 1 || true)"
    if [[ -n "$WINDOW" ]]; then
        break
    fi
    sleep 1
done

if [[ -z "$WINDOW" ]]; then
    echo "Could not find LIRUVA's visible window."
    cat "$APP_LOG"
    exit 1
fi

# Allow the real eframe app to paint its initial frame before capture.
sleep 3
import -window "$WINDOW" "$OUTPUT"
test -s "$OUTPUT"
file "$OUTPUT"
echo "Captured the actual LIRUVA desktop window."
