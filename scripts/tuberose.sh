#!/bin/bash

set -e

. /etc/os-release

export WORKSPACE=$PWD

if [[ $ID == "ubuntu" ]]; then
    apt update
    apt -y upgrade
    DEBIAN_FRONTEND=noninteractive apt install -y git cmake \
        clang mold \
        build-essential crossbuild-essential-arm64 crossbuild-essential-riscv64
elif [[ $ID == "arch" ]]; then
    echo "build on archlinux"
else
    echo "unsupported os($ID)"
    exit 1
fi

function build_tuberose() {
    local thrift_args="-DBUILD_TESTING=OFF -DBUILD_COMPILER=OFF -DWITH_OPENSSL=OFF -DBUILD_JAVA=OFF -DBUILD_JAVASCRIPT=OFF -DBUILD_NODEJS=OFF -DBUILD_PYTHON=OFF"
    local mailio_args="-DMAILIO_BUILD_DOCUMENTATION=OFF -DMAILIO_BUILD_EXAMPLES=OFF -DMAILIO_BUILD_SHARED_LIBRARY=OFF -DMAILIO_BUILD_TESTS=OFF"
    local casbin_args="-DCASBIN_BUILD_TEST=OFF -DCASBIN_BUILD_BINDINGS=OFF -DCASBIN_BUILD_BENCHMARK=OFF -DCASBIN_BUILD_PYTHON_BINDINGS=OFF -DCASBIN_INSTALL=OFF"
    local build_dir=$WORKSPACE/build/tuberose-$1
    mkdir -p $build_dir

    cmake -S $WORKSPACE/tuberose -B $build_dir -DCMAKE_MAKE_PROGRAM=make -DCMAKE_BUILD_TYPE=Release \
        -DVCPKG_HOST_TRIPLET=x64-linux-release -DVCPKG_TARGET_TRIPLET=$1-linux-release \
        -DCMAKE_TOOLCHAIN_FILE=$WORKSPACE/tuberose/vcpkg/scripts/buildsystems/vcpkg.cmake \
        -DVCPKG_CHAINLOAD_TOOLCHAIN_FILE=$WORKSPACE/tuberose/toolchains/$1.cmake \
        $mailio_args $casbin_args $thrift_args -DBoost_NO_WARN_NEW_VERSIONS=1

    # make -j -C $build_dir

}

declare -a triplets=(
    "x64"
    "arm64"
    # "riscv64"
)
for t in "${triplets[@]}"; do
    build_tuberose $t
done

echo 'done.'
exit 0
