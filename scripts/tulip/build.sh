#!/bin/bash

set -e

export SOURCE_DIR=$PWD
export BUILD_DIR=$SOURCE_DIR/build/$(uname -m)

if [ ! -d $BUILD_DIR ]
then
    mkdir -p $BUILD_DIR
fi

cmake -S $SOURCE_DIR -B $BUILD_DIR -DCMAKE_BUILD_TYPE=Release -G "Ninja" \
    -DTINK_USE_SYSTEM_OPENSSL=ON
cmake --build $BUILD_DIR

echo 'done.'
exit 0
