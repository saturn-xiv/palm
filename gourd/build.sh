#!/bin/bash

set -e

export SOURCE_ROOT=$PWD
export BUILD_ROOT=$PWD/build/Release

mkdir -p $BUILD_ROOT
CC=gcc-12 CXX=g++-12 cmake -DCMAKE_BUILD_TYPE=Release \
    -DTINK_USE_SYSTEM_OPENSSL=ON -DTINK_BUILD_TESTS=OFF \
    -B $BUILD_ROOT -S $SOURCE_ROOT
make -C $BUILD_ROOT loquat

echo 'done.'
exit 0
