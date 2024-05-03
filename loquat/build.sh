#!/bin/bash

set -e

. /etc/os-release

# https://github.com/grpc/grpc/blob/master/BUILDING.md#pre-requisites
if [[ "$ID" == "ubuntu" ]] && [[ "$VERSION_CODENAME" == "jammy" ]]; then
    apt update
    apt install -y curl git zip \
        pkg-config build-essential g++-12 cmake flex bison \
        linux-libc-dev
    update-alternatives --install /usr/bin/g++ g++ /usr/bin/g++-12 100
    update-alternatives --install /usr/bin/gcc gcc /usr/bin/gcc-12 100
elif [[ "$ID" == "arch" ]]; then
    echo 'building on arch'
else 
    echo "Unsupported system: $ID/$VERSION_CODENAME"
    exit 1
fi

export SOURCE_ROOT=$PWD
export BUILD_ROOT=$PWD/build/Release

mkdir -p $BUILD_ROOT
cmake -DCMAKE_BUILD_TYPE=Release \
    -DABSL_PROPAGATE_CXX_STD=ON \
    -DTINK_USE_SYSTEM_OPENSSL=ON -DTINK_BUILD_TESTS=OFF \
    -B $BUILD_ROOT -S $SOURCE_ROOT
make -C $BUILD_ROOT loquat

echo 'done.'
exit 0
