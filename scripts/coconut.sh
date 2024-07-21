#!/bin/bash

set -e

export $WORKSPACE=$PWD

function build_coconut() {
    local build_dir=$WORKSPACE/build/$1
    mkdir -p $build_dir
    CXX=$1-linux-musl-g++ cmake -S $WORKSPACE/coconut -B $build_dir -DCMAKE_BUILD_TYPE=Release
    make -C $build_dir coconut
}

declare -a triples=(
    "x86_64"
    "aarch64"
    "armv7l"
    # "riscv64"
)
for t in "${triples[@]}"; do
    build_coconut $t
done

echo 'done.'
exit 0
