#!/bin/bash

set -e

. /etc/os-release

if [[ $ID != "ubuntu" ]]; then
    echo "unsupported os($PRETTY_NAME)"
    exit 1
fi

apt update
apt -y upgrade

DEBIAN_FRONTEND=noninteractive apt install -y build-essential git cmake ninja-build g++ pkg-config \
    libssl-dev libboost-all-dev

export WORK_DIR=$PWD
export BUILD_DIR=$WORK_DIR/build/$VERSION_CODENAME-$(uname -m)

mkdir -p $BUILD_DIR

# https://github.com/libevent/libevent/blob/release-2.1.12-stable/cmake/LibeventConfig.cmake.in#L22
cmake -S $WORK_DIR -B $BUILD_DIR -DCMAKE_BUILD_TYPE=Release -G Ninja \
    -DOPENSSL_USE_STATIC_LIBS=ON \
    -DLIBEVENT_STATIC_LINK=TRUE -DEVENT__LIBRARY_TYPE=STATIC -DEVENT__DISABLE_DEBUG_MODE=ON -DEVENT__DISABLE_TESTS=ON -DEVENT__DISABLE_SAMPLES=ON \
    -DNINJA_BUILD_BINARY=OFF -DBUILD_TESTING=OFF \
    -DABSL_PROPAGATE_CXX_STD=ON -DTINK_USE_SYSTEM_OPENSSL=ON \
    -DBUILD_COMPILER=OFF -DWITH_OPENSSL=ON -DBUILD_JAVA=OFF -DBUILD_JAVASCRIPT=OFF -DBUILD_NODEJS=OFF -DBUILD_PYTHON=OFF

cmake --build $BUILD_DIR

echo 'done.'
exit 0
