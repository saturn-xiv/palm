#!/bin/bash

set -e

export VCPKG_DEFAULT_BINARY_CACHE=$PWD/.cache
mkdir -p $VCPKG_DEFAULT_BINARY_CACHE

declare -a targets=("x86_64" "aarch64" "riscv64")
for i in "${targets[@]}"
do
   cmake --preset=$i
   cmake --build build/$i
done

echo 'done.'
exit 0
