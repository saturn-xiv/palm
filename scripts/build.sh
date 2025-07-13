#!/bin/bash

set -e

# https://github.com/casbin/casbin-cpp?tab=readme-ov-file#without-installing-casbin-locally
export CASBIN_FLAGS="-DCASBIN_INSTALL=OFF -DCASBIN_BUILD_TEST=OFF -DCASBIN_BUILD_BENCHMARK=OFF -DCASBIN_BUILD_BINDINGS=OFF -DCASBIN_BUILD_PYTHON_BINDINGS=OFF"
export THRIFT_FLAGS="-DCMAKE_BUILD_TYPE=Release -DBUILD_COMPILER=OFF -DWITH_OPENSSL=ON -DBUILD_JAVA=OFF -DBUILD_JAVASCRIPT=OFF -DBUILD_NODEJS=OFF -DBUILD_PYTHON=OFF"
# -DBOOST_DATE_TIME_POSIX_TIME_STD_CONFIG
export BOOST_FLAGS=""
export WORK_DIR=$PWD

. /etc/os-release

if [ $ID == "arch" ]; then
    cmake --preset=arch -DVCPKG_TARGET_TRIPLET=x64-linux-release -DVCPKG_CHAINLOAD_TOOLCHAIN_FILE=$WORK_DIR/toolchains/arch/clang.cmake $BOOST_FLAGS $THRIFT_FLAGS $CASBIN_FLAGS
    cmake --build $WORK_DIR/build/arch
elif [ $ID == "ubuntu" ]; then
    cmake --preset=x86_64 -DVCPKG_TARGET_TRIPLET=x64-linux-release $BOOST_FLAGS $THRIFT_FLAGS $CASBIN_FLAGS
    cmake --build $WORK_DIR/build/x86_64
    # cmake --preset=x86_64 -DVCPKG_TARGET_TRIPLET=x64-linux-release -DVCPKG_CHAINLOAD_TOOLCHAIN_FILE=$WORK_DIR/toolchains/ubuntu/clang/x86_64.cmake $THRIFT_FLAGS
    # cmake --build $WORK_DIR/build/x86_64

    cmake --preset=aarch64 -DVCPKG_TARGET_TRIPLET=arm64-linux-release -DVCPKG_CHAINLOAD_TOOLCHAIN_FILE=$WORK_DIR/toolchains/ubuntu/gcc/aarch64.cmake $BOOST_FLAGS $THRIFT_FLAGS $CASBIN_FLAGS
    cmake --build $WORK_DIR/build/aarch64
else
    echo "unsupported os $PRETTY_NAME"
fi

echo "done($PRETTY_NAME)"
exit 0
