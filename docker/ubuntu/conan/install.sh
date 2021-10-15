#!/bin/bash

set -e


declare -a profiles=(
    "amd64-linux"
    # "armhf-linux"
    # "arm64-linux"
)

for p in "${profiles[@]}"
do
    conan install --build --profile:build=default --profile:host=$p .
done

echo 'done'

exit 0
