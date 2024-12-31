#!/usr/bin/env bash

SCRIPT_DIR=$(dirname "$(realpath "$0")")
echo "The script is located in: $SCRIPT_DIR"

export PKG_CONFIG_PATH=/usr/aarch64-linux-gnu/lib/pkgconfig:/usr/lib/aarch64-linux-gnu/pkgconfig

pushd "$SCRIPT_DIR"

rustup target add aarch64-unknown-linux-gnu
cargo build --target aarch64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu


popd
