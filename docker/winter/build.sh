#!/bin/bash

set -e

export CODE="palm-winter"

ARCH=$(uname -m)

docker pull ubuntu:latest

if [ "$ARCH" = "x86_64" ]; then
    docker build --network host --platform=linux/amd64 --provenance false -t $CODE .
elif [ "$ARCH" = "aarch64" ]; then
    docker build --network host --platform=linux/arm64 --provenance false -t $CODE .
else
    echo "unsupported $ARCH"
    exit 1
fi

echo "done."

exit 0
