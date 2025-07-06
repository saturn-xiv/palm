#!/bin/bash

set -e

export THRIFT_FLAGS="-DCMAKE_BUILD_TYPE=Release -DBUILD_COMPILER=OFF -DWITH_OPENSSL=ON -DBUILD_JAVA=OFF -DBUILD_JAVASCRIPT=OFF -DBUILD_NODEJS=OFF -DBUILD_PYTHON=OFF"

function build_for_x64() {
    cmake --preset=x86_64 -DVCPKG_TARGET_TRIPLET=x64-linux-release $THRIFT_FLAGS
    cmake --build
}

function build_for_arm64() {
    cmake --preset=aarch64 -DVCPKG_TARGET_TRIPLET=arm64-linux-release -DVCPKG_CHAINLOAD_TOOLCHAIN_FILE=$PWD/toolchains/gcc/aarch64.cmake $THRIFT_FLAGS
    cmake --build
}

. /etc/os-release
if [ $ID == "arch"]; then
    # lib32-glibc
    build_for_x64
elif [$ID == "ubuntu"]; then
    build_for_x64
    build_for_arm64
else
    echo "unsupported arch $ID"
fi

echo "done($1)"
exit 0
