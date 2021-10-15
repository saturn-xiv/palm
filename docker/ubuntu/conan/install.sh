#!/bin/bash

set -e



export OS_NAME=$(lsb_release -is)

if [[ $OS_NAME == "Ubuntu" ]]
then
    declare -a profiles=(
        "amd64-linux"
        "armhf-linux"
        "arm64-linux"
    )
    for p in "${profiles[@]}"
    do
        conan install --build=missing --profile:build=default --profile:host=$p .
    done    
elif [[ $OS_NAME == "Arch" ]]
then
    conan install --build=missing --profile:build=default --profile:host=archlinux .
else
    echo "Unknowk os $OS_NAME"
    exit 1
fi


echo 'done'

exit 0
