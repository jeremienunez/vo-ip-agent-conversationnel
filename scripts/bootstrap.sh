#!/usr/bin/env bash
set -euo pipefail

if ! command -v rustup >/dev/null 2>&1; then
  echo "rustup is required. Install from https://rustup.rs before running bootstrap." >&2
  exit 1
fi

TOOLCHAIN=$(awk -F '"' '/channel/ { print $2 }' rust-toolchain.toml)
rustup toolchain install "$TOOLCHAIN"
rustup component add --toolchain "$TOOLCHAIN" rustfmt clippy miri

if ! command -v cargo-tarpaulin >/dev/null 2>&1; then
  cargo install cargo-tarpaulin
fi

echo "Bootstrap complete."
