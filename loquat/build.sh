#!/bin/bash

set -e

. /etc/os-release

if [[ $ID != "ubuntu" ]]; then
    echo "unsupported os($PRETTY_NAME)"
    exit 1
fi

apt update
apt -y upgrade
# libunwind-dev golang
DEBIAN_FRONTEND=noninteractive apt install -y build-essential git cmake ninja-build g++ \
    libssl-dev libevent-dev libboost-all-dev

export WORK_DIR=$PWD
export BUILD_DIR=$WORK_DIR/build/$VERSION_CODENAME-$(uname -m)

mkdir -p $BUILD_DIR

cmake -S $WORK_DIR -B $BUILD_DIR -DCMAKE_BUILD_TYPE=Release -G Ninja \
    -DNINJA_BUILD_BINARY=OFF -DBUILD_TESTING=OFF \
    -DABSL_PROPAGATE_CXX_STD=ON -DTINK_USE_SYSTEM_OPENSSL=ON \
    -DBUILD_COMPILER=OFF -DWITH_OPENSSL=ON -DBUILD_JAVA=OFF -DBUILD_JAVASCRIPT=OFF -DBUILD_NODEJS=OFF -DBUILD_PYTHON=OFF

cmake --build $BUILD_DIR

echo 'done.'
exit 0
