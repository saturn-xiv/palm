#!/bin/bash

set -e

. /etc/os-release

export VCPKG_DISABLE_METRICS=1
export VCPKG_DEFAULT_BINARY_CACHE=$PWD/.cache

# https://github.com/grpc/grpc/blob/master/BUILDING.md#pre-requisites
if [[ "$ID" == "ubuntu" ]]; then
    apt update
    apt -y upgrade
    DEBIAN_FRONTEND=noninteractive apt install -y wget curl git zip \
        pkg-config build-essential cmake ninja-build flex bison \
	autoconf autoconf-archive automake libtool \
	crossbuild-essential-amd64 crossbuild-essential-arm64 crossbuild-essential-riscv64 \
	g++-x86-64-linux-gnu g++-aarch64-linux-gnu g++-riscv64-linux-gnu
fi

mkdir -p $VCPKG_DEFAULT_BINARY_CACHE

# https://github.com/tink-crypto/tink-cc/blob/main/cmake/TinkWorkspace.cmake
declare -a targets=("x86_64" "aarch64" "riscv64")
for i in "${targets[@]}"
do
   cmake -DTINK_USE_INSTALLED_ABSEIL=ON -DTINK_USE_INSTALLED_PROTOBUF=ON -DTINK_USE_SYSTEM_OPENSSL=ON -DTINK_BUILD_TESTS=OFF --preset=$i
   cmake --build build/$i
done

echo 'done.'
exit 0
