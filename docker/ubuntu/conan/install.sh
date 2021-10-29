#!/bin/bash

set -e

export OS_NAME=$(lsb_release -is)

if [[ $OS_NAME == "Ubuntu" ]]
then
    declare -a profiles=(
        "libstdc++/amd64"
        "libstdc++/arm64"
        "libstdc++/armhf"
        "libc++/amd64"
        "libc++/arm64"
        "libc++/armhf"
    )
elif [[ $OS_NAME == "Arch" ]]
then
    declare -a profiles=(
        "libstdc++/arch"
        "libc++/arch"
    )
else
    echo "Unknowk os $OS_NAME"
    exit 1
fi

for p in "${profiles[@]}"
do
    conan install --build --profile:build=default --profile:host=./profiles/$p .
done    

echo 'done.'

exit 0
