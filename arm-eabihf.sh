#/usr/bin/env bash

export RUSTFLAGS='-C target-feature=-crt-static -L native=/usr/arm-linux-gnueabihf/lib -L native=/home/asonix/Development/armv7h/lib'
export OPENSSL_DIR=/home/asonix/Development/armv7h
export PKG_CONFIG_ALLOW_CROSS=1
cargo build --target arm-unknown-linux-gnueabihf --release
