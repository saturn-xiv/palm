#!/bin/bash

set -e

. /etc/os-release

# https://github.com/grpc/grpc/blob/master/BUILDING.md#pre-requisites
if [[ "$ID" == "ubuntu" ]] && [[ "$VERSION_CODENAME" == "jammy" ]]; then
    apt update
    apt install -y git build-essential cmake autoconf libtool pkg-config \
        g++-12 golang \
        python3 python3-pip \
        libssl-dev libunwind-dev libevent-dev libboost-all-dev
else
    echo "Unsupported system: $ID/$VERSION_CODENAME"
    exit 1
fi

pip3 install cmake

export SOURCE_ROOT=$PWD
export BUILD_ROOT=$PWD/build/Release

mkdir -p $BUILD_ROOT

CC=gcc-12 CXX=g++-12 cmake -DCMAKE_BUILD_TYPE=Release \
    -DABSL_PROPAGATE_CXX_STD=ON \
    -DgRPC_INSTALL=ON -DgRPC_SSL_PROVIDER=package -DgRPC_BUILD_TESTS=OFF \
    -DTINK_USE_SYSTEM_OPENSSL=ON -DTINK_BUILD_TESTS=OFF -DTINK_USE_INSTALLED_PROTOBUF=ON -DTINK_USE_INSTALLED_ABSEIL=ON \
    -DBUILD_SHARED_LIBS=OFF -DBUILD_TESTING=OFF -DBUILD_COMPILER=OFF -DWITH_OPENSSL=ON -DBUILD_JAVA=OFF -DBUILD_JAVASCRIPT=OFF -DBUILD_NODEJS=OFF -DBUILD_PYTHON=OFF \
    -DCASBIN_BUILD_TESTS=OFF -DCASBIN_BUILD_PYTHON_BINDINGS=OFF \
    -DLEVELDB_BUILD_TESTS=OFF -DLEVELDB_BUILD_BENCHMARKS=OFF \
    -B $BUILD_ROOT -S $SOURCE_ROOT

make -C $BUILD_ROOT loquat

echo 'done.'
exit 0
