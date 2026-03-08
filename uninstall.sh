#!/bin/sh

set -eu

INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"
ZISM_BIN="${INSTALL_DIR}/zism"

if [ -f "$ZISM_BIN" ]; then
  rm -f "$ZISM_BIN"
  printf "Removed %s\n" "$ZISM_BIN"
else
  printf "zism not found at %s\n" "$ZISM_BIN"
fi

printf "Done.\n"
