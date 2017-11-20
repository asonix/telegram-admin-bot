#!/usr/bin/env bash

export RUSTFLAGS='-C target-feature=-crt-static -L native=/usr/aarch64-linux-gnu/lib -L native=/home/asonix/Development/aarch64/lib'
export OPENSSL_DIR=/home/asonix/Development/aarch64
export PKG_CONFIG_ALLOW_CROSS=1
cargo build --target aarch64-unknown-linux-gnu --release
