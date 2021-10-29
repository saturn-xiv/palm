#!/bin/bash

set -e

export OS_NAME=$(lsb_release -is)

if [[ $OS_NAME == "Ubuntu" ]]
then
    declare -a profiles=(
        "amd64-clang"
        "armhf-clang"
        "arm64-clang"
    )
    for p in "${profiles[@]}"
    do
        conan install --build --profile:build=default --profile:host=./profiles/$p .
    done    
elif [[ $OS_NAME == "Arch" ]]
then
    conan install --build --profile:build=default --profile:host=./profiles/arch-clang .
else
    echo "Unknowk os $OS_NAME"
    exit 1
fi


echo 'done.'

exit 0
