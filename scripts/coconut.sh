#!/bin/bash

set -e

. /etc/os-release

export WORKSPACE=$PWD

function build_on_ubuntu() {
    local build_dir=$WORKSPACE/build/coconut-$1
    mkdir -p $build_dir
    CXX=$1-linux-musl-g++ cmake -S $WORKSPACE/coconut -B $build_dir -DCMAKE_BUILD_TYPE=Release
    make -j -C $build_dir coconut
}

function build_on_arch() {
    local build_dir=$WORKSPACE/build/coconut
    mkdir -p $build_dir
    CXX=clang++ cmake -S $WORKSPACE/coconut -B $build_dir -DCMAKE_BUILD_TYPE=Release
    make -j -C $build_dir coconut
}

if [[ $ID == "ubuntu" ]]; then
    apt update
    apt -y upgrade
    DEBIAN_FRONTEND=noninteractive apt install -y build-essential git cmake
    declare -a triples=(
        "x86_64"
        "aarch64"
        "armv7l"
        # "riscv64"
    )
    for t in "${triples[@]}"; do
        build_on_ubuntu $t
    done
elif [[ $ID == "arch" ]]; then
    build_on_arch
else
    echo "unsupported os($ID)"
    exit 1
fi

echo 'done.'
exit 0