#!/bin/bash

set -e

export WORKSPACE=$PWD

declare -a profiles=(
    "amd64"
    "arm64"
    "armhf"
)

function build_conan() {
    local target=$HOME/build/conan/$1
    if [ -d $target ]
    then
        rm -rv $target
    fi
    mkdir -pv $target
    cd target
    conan install --build=missing \
        --profile:build=default \
        --profile:host=$WORKSPACE/docker/crosstool-ng/profiles/$1 \
        $WORKSPACE/docker
}

for p in "${profiles[@]}"
do
    build_conan $p
done    

echo 'done.'
exit 0
