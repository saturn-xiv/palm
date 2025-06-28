#!/bin/bash

set -e

if [ -d build ]; then
    rm -r build
fi

# xmake show -l toolchains

echo "build for x86_64"
xmake f --toolchain=clang-19 -p linux -a x86_64 -m release -c
xmake

echo "build for aarch64"
xmake f -p cross --cross=aarch64-linux-gnu- --cc=aarch64-linux-gnu-gcc-14 --cxx=aarch64-linux-gnu-g++-14 -m release -c
xmake

echo "build for riscv64"
xmake f -p cross --cross=riscv64-linux-gnu- --cc=riscv64-linux-gnu-gcc-14 --cxx=riscv64-linux-gnu-g++-14 -m release -c
xmake

echo "done."
