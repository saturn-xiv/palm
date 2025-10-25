#!/bin/bash

set -e

export SOURCE_DIR=$PWD
export BUILD_DIR=$SOURCE_DIR/build/$(uname -m)

cmake --preset=x86_64 -DVCPKG_TARGET_TRIPLET=x64-linux-release
cmake --build $PWD/build/x86_64

cmake --preset=aarch64 -DVCPKG_TARGET_TRIPLET=arm64-linux-release
cmake --build $PWD/build/aarch64

echo 'done.'
exit 0
