#!/bin/bash

set -e

declare -a targets=(
    "x86_64-unknown-linux-gnu"
    "x86_64-unknown-linux-musl"
    "armv7-unknown-linux-gnueabihf"
    "armv7-unknown-linux-musleabihf"
)

for t in "${targets[@]}"
do
    cargo build --target $t --release
done

echo 'done'
exit 0
