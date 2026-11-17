#!/usr/bin/env bash
# Prints the version Cargo.toml gives the compile-outcomes package, the workspace's
# version, which every published crate shares.
set -euo pipefail
cargo metadata --no-deps --format-version 1 \
    | jq -r '.packages[] | select(.name == "compile-outcomes") | .version' \
    | tr -d '\r'
