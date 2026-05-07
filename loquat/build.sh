#!/bin/bash

set -e

. /etc/os-release

# https://github.com/grpc/grpc/blob/master/BUILDING.md#pre-requisites
if [[ "$ID" == "ubuntu" ]] && [[ "$VERSION_CODENAME" == "jammy" ]]; then
    apt update
    apt -y upgrade
    apt install -y curl git zip \
        pkg-config build-essential g++-12 cmake ninja-build flex bison \
        linux-libc-dev libssl-dev libenvent-dev libboost-all-dev
elif [[ "$ID" == "arch" ]]; then
    echo 'building on arch'
else
    echo "Unsupported system: $ID/$VERSION_CODENAME"
    exit 1
fi

export SOURCE_ROOT=$PWD
export BUILD_ROOT=$PWD/build/Release

mkdir -p $BUILD_ROOT
CC=gcc-12 CXX=g++-12 cmake -DCMAKE_BUILD_TYPE=Release \
    -DABSL_PROPAGATE_CXX_STD=ON \
    -DTINK_USE_SYSTEM_OPENSSL=ON -DTINK_BUILD_TESTS=OFF \
    -B $BUILD_ROOT -S $SOURCE_ROOT \
    -G Ninja
cmake --build $BUILD_ROOT

echo 'done.'
exit 0
