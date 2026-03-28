#!/bin/bash

set -e

# "riscv64"
declare -a targets=("x86_64" "aarch64")
for i in "${targets[@]}"
do
   cmake --preset=$i
   cmake --build build/$i
done

echo 'done.'
exit 0
