#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 1 ]]; then
    echo "usage: $(basename "$0") <binary> [args...]" >&2
    exit 1
fi

binary=$(realpath "$1")
shift || true

hatari-prg-args -q \
  --tos ~/.config/hatari/emutos-512k/etos512us.img \
  --conout 2 \
  -- "$binary" "$@"
