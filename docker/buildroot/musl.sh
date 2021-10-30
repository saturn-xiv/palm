#!/bin/bash

set -e

# https://musl.cc/#binaries
function build_toolchain() {
    local target=$HOME/local/$1
    if [ -d $target/bin ]
    then
        echo "toolchain $1 already exists"
        return
    fi
    local workspace=$HOME/build/musl-cross-make
    
    if [ -d $workspace ]
    then
        cd $workspace
        git pull
    else
        git clone https://github.com/richfelker/musl-cross-make.git $workspace
    fi

    cd $workspace
    echo "TARGET = $1" > config.mak
    make
    make install
    mv output $target
}

declare -a targets=(
    "x86_64-linux-musl"
    "aarch64-linux-musl"
    "arm-linux-musleabihf"
)

for t in "${targets[@]}"
do
    build_toolchain $t
done

echo 'done.'
exit 0
