#!/bin/sh

PKG_CONFIG_ALLOW_CROSS=1
PKG_CONFIG_DIR=
PKG_CONFIG_LIBDIR=/usr/lib/arm-linux-gnueabihf/pkgconfig
export PKG_CONFIG_ALLOW_CROSS PKG_CONFIG_DIR PKG_CONFIG_LIBDIR

# RUST_LOG=debug ./grpc_test-54ab9e42f0ef9c3f --test client --nocapture
cargo test --no-run --target armv7-unknown-linux-gnueabihf --test $1
