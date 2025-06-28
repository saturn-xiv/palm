#!/bin/bash

set -e

. /etc/os-release

if [[ $ID != "ubuntu" ]]; then
    echo "unsupported os($PRETTY_NAME)"
    exit 1
fi

export WORK_DIR=$PWD

function build_library() {
    local build_dir=$WORK_DIR/build/$VERSION_CODENAME-$1
    mkdir -p $build_dir

    cmake -S $WORK_DIR -B $build_dir -DCMAKE_TOOLCHAIN_FILE=$(dirname $WORK_DIR)/toolchains/gcc/$1.cmake \
        -DCMAKE_BUILD_TYPE=Release -G Ninja \
        -DABSL_PROPAGATE_CXX_STD=ON -DTINK_USE_SYSTEM_OPENSSL=OFF

    cmake --build $build_dir
}

# build_library x86_64
build_library aarch64
build_library riscv64

echo 'done.'
exit 0
