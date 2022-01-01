#!/bin/bash

set -e

declare -a triples=(
    "x86_64-unknown-linux-gnu"
    "aarch64-unknown-linux-gnu"
    "armv8-rpi3-linux-gnueabihf"
)

function build_toolchain(){
    local target=$HOME/build/toolchains/$1
    if [ ! -d $target ]
    then
        mkdir -pv $target
    fi
    cp -v $1 $target/.config
    cd $target
    ct-ng build
}

for t in "${triples[@]}"
do
    build_toolchain $t
done

echo 'done.'
exit 0
