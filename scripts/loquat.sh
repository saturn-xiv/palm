#!/bin/bash

set -e

. /etc/os-release

export GCC_VERSION=12

if [[ $ID == "ubuntu" ]]; then
    apt update
    apt -y upgrade
    DEBIAN_FRONTEND=noninteractive apt install -y build-essential git cmake g++-$GCC_VERSION golang \
        libssl-dev libevent-dev libboost-all-dev libunwind-dev
elif [[ $ID == "arch" ]]; then
    echo "build on archlinux"
else
    echo "unsupported os($ID)"
    exit 1
fi

export WORKSPACE=$PWD
export BUILD_DIR=$WORKSPACE/build/loquat

mkdir -p $BUILD_DIR

CC=gcc-$GCC_VERSION CXX=g++-$GCC_VERSION cmake -S $WORKSPACE/loquat -B $BUILD_DIR -DCMAKE_BUILD_TYPE=Release \
    -DABSL_PROPAGATE_CXX_STD=ON -DTINK_USE_SYSTEM_OPENSSL=ON \
    -DBUILD_COMPILER=OFF -DWITH_OPENSSL=ON -DBUILD_JAVA=OFF -DBUILD_JAVASCRIPT=OFF -DBUILD_NODEJS=OFF -DBUILD_PYTHON=OFF

make -j -C $BUILD_DIR loquat

echo 'done.'
exit 0
