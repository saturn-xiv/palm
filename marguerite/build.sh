#!/bin/bash

set -e

. /etc/os-release

if [[ "$ID" == "ubuntu" ]] && [[ "$VERSION_CODENAME" == "jammy" ]]; then
    echo "Building on $VERSION_CODENAME"
else
    echo "Unsupported system: $ID/$VERSION_CODENAME"
    exit 1
fi

export SOURCE_ROOT=$PWD
export BUILD_ROOT=$PWD/build/Release

export VCPKG_ROOT=$HOME/local/vcpkg
export VCPKG_DISABLE_METRICS=1
if [ ! -d $VCPKG_ROOT ]; then
    git clone -b 2025.01.13 https://github.com/microsoft/vcpkg.git $VCPKG_ROOT
    $VCPKG_ROOT/bootstrap-vcpkg.sh
fi

#
# -DVCPKG_CHAINLOAD_TOOLCHAIN_FILE=$WORKSPACE/toolchains/$ID/$1.cmake \
# -DVCPKG_HOST_TRIPLET=x64-linux-release -DVCPKG_TARGET_TRIPLET=$1-linux-release \
mkdir -p $BUILD_ROOT
cmake -DCMAKE_C_COMPILER=gcc-12 -DCMAKE_CXX_COMPILER=g++-12 -DCMAKE_BUILD_TYPE=Release \
    -DCMAKE_TOOLCHAIN_FILE=$VCPKG_ROOT/scripts/buildsystems/vcpkg.cmake \
    -DABSL_PROPAGATE_CXX_STD=ON \
    -DTINK_USE_SYSTEM_OPENSSL=ON -DTINK_BUILD_TESTS=OFF \
    -DCURLPP_BUILD_SHARED_LIBS=OFF \
    -B $BUILD_ROOT -S $SOURCE_ROOT -G Ninja
ninja -C $BUILD_ROOT

echo 'Done.'
exit 0
