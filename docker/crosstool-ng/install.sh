#!/bin/bash

set -e

export WORKSPACE=$PWD

declare -a triples=(
    "x86_64-unknown-linux-gnu"
    "aarch64-unknown-linux-gnu"
    "armv7-rpi2-linux-gnueabihf"
)

function build_toolchain(){
    if [ -f $HOME/x-tools/$1/build.log.bz2 ]
    then
        echo "ignore $1"
        return
    fi
    if [ -d $HOME/x-tools/$1 ]
    then
        rm -r $HOME/x-tools/$1
    fi
    local target=$HOME/build/toolchains/$1
    if [ ! -d $target ]
    then
        mkdir -pv $target
    fi
    cp -v $WORKSPACE/docker/crosstool-ng/$1 $target/.config
    cd $target
    ct-ng build
}

for t in "${triples[@]}"
do
    build_toolchain $t
done

echo 'done.'
exit 0
