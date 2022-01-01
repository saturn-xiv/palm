#!/bin/bash

set -e

export WORKSPACE=$PWD
export OS_NAME=$(lsb_release -is)

function build_conan() {
    local target=$HOME/build/conan/$1
    if [ -d $target ]
    then
        rm -rv $target
    fi
    mkdir -pv $target
    cd $target
    conan install --build=missing \
        --profile:build=default \
        --profile:host=$WORKSPACE/docker/crosstool-ng/profiles/$1 \
        $WORKSPACE/docker
}

if [[ $OS_NAME == "Ubuntu" ]]
then
    declare -a profiles=(
        "amd64"
        "arm64"
        "armhf"
    )
    for p in "${profiles[@]}"
    do
        build_conan $p
    done   
elif [[ $OS_NAME == "Arch" ]]
    build_conan "arch"
then
else
    echo "unknown os $OS_NAME"
    exit 1
fi

echo 'done.'
exit 0
