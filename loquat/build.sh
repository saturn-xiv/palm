#!/bin/bash

set -e

. /etc/os-release

if [[ $ID != "ubuntu" ]]; then
    echo "unsupported os($PRETTY_NAME)"
    exit 1
fi

export WORK_DIR=$PWD
export BUILD_DIR=$WORK_DIR/build/$VERSION_CODENAME-$(uname -m)

mkdir -p $BUILD_DIR

cmake -S $WORK_DIR -B $BUILD_DIR -DCMAKE_BUILD_TYPE=Release -G Ninja \
    -DABSL_PROPAGATE_CXX_STD=ON -DTINK_USE_SYSTEM_OPENSSL=OFF

cmake --build $BUILD_DIR

echo 'done.'
exit 0
