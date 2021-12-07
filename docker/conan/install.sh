#!/bin/bash

set -e

export WORKSPACE=$PWD
export OS_NAME=$(lsb_release -is)

if [[ $OS_NAME == "Ubuntu" ]]
then
    # sudo apt install -y libc++-13-dev libc++abi-13-dev
    declare -a profiles=(
        "libstdc++/amd64"
        "libstdc++/arm64"
        "libstdc++/armhf"
        # "libc++/amd64"
        # "libc++/arm64"
        # "libc++/armhf"
    )
elif [[ $OS_NAME == "Arch" ]]
then
    declare -a profiles=(
        "libstdc++/arch"
        # "libc++/arch"
    )
else
    echo "Unknowk os $OS_NAME"
    exit 1
fi

for p in "${profiles[@]}"
do
    mkdir -pv $HOME/build/$p
    cd $HOME/build/$p
    conan install --build=missing --profile:build=default --profile:host=$WORKSPACE/profiles/$p $WORKSPACE
done    

echo 'done.'

exit 0
