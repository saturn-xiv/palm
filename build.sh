#!/bin/bash

set -e

source /etc/os-release

export WORK_DIR=$PWD
export TARGET_DIR=$WORK_DIR/tmp
export PACKAGE=palm-$ID-$VERSION_CODENAME-$(uname -m)-$(git describe --tags --always --dirty)

# ---------------------------------------------------------

function build_loquat() {
    local source_root=$WORK_DIR/loquat
    local build_root=$source_root/build/Release-$ID-$VERSION_CODENAME-$(uname -m)
    
    cmake -Wno-dev -DCMAKE_BUILD_TYPE=Release \
        -DABSL_PROPAGATE_CXX_STD=ON \
        -DTINK_USE_SYSTEM_OPENSSL=ON -DTINK_BUILD_TESTS=OFF \
        -B $build_root -S $source_root \
        -G Ninja
    cmake --build $build_root

    mkdir -p $TARGET_DIR/$PACKAGE/bin
    cp $build_root/loquat $TARGET_DIR/$PACKAGE/bin/
}

# ---------------------------------------------------------

build_loquat

XZ_OPT=-9 tar -cJf $WORK_DIR/$PACKAGE.tar.xz -C $WORK_DIR/$PACKAGE .

echo "done($PACKAGE)."
exit 0
