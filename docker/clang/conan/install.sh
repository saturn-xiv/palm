#!/bin/bash

set -e

export WORKSPACE=$PWD

declare -a profiles=(
    "amd64"
    "arm64"
    "armhf"
)

for p in "${profiles[@]}"
do
    mkdir -pv $HOME/build/palm-$p
    cd $HOME/build/palm-$p
    conan install --build --profile:build=default --profile:host=$WORKSPACE/profiles/$p $WORKSPACE
done    

echo 'done.'

exit 0
