#!/bin/bash

set -e

. /etc/os-release

# https://github.com/grpc/grpc/blob/master/BUILDING.md#pre-requisites
if [[ "$ID" == "ubuntu" ]]; then
    apt update
    apt -y upgrade
    DEBIAN_FRONTEND=noninteractive apt install -y curl git zip \
        pkg-config build-essential g++-12 cmake ninja-build flex bison \
        linux-libc-dev libssl-dev libevent-dev libboost-all-dev
else
    echo "Unsupported system: $ID/$VERSION_CODENAME"
    exit 1
fi

export SOURCE_ROOT=$PWD
export BUILD_ROOT=$PWD/build/Release-$VERSION_CODENAME-$(uname -m)

mkdir -p $BUILD_ROOT
# -Wno-dev
CC=gcc-12 CXX=g++-12 cmake -DCMAKE_BUILD_TYPE=Release \
    -DBUILD_SHARED_LIBS=OFF -DBoost_USE_STATIC_LIBS=ON -DBUILD_COMPILER=OFF -DWITH_LIBEVENT=ON -DBUILD_CPP=ON -DWITH_JAVA=OFF -DWITH_PYTHON=OFF -DWITH_NODEJS=OFF -DWITH_JAVASCRIPT=OFF \
    -DABSL_PROPAGATE_CXX_STD=ON \
    -DTINK_USE_SYSTEM_OPENSSL=ON -DTINK_BUILD_TESTS=OFF \
    -B $BUILD_ROOT -S $SOURCE_ROOT \
    -G Ninja
cmake --build $BUILD_ROOT

echo 'done.'
exit 0
