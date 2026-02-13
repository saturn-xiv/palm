#!/bin/bash

set -e

export VERSION=$(date "+%4Y%m%d%H%M%S")
export CODE="palm-autumn"

ARCH=$(uname -m)

docker pull ubuntu:latest

if [ "$ARCH" = "x86_64" ]; then
    docker build --platform=linux/amd64 --provenance false -t $CODE .
elif [ "$ARCH" = "aarch64" ]; then
    docker build --platform=linux/arm64 --provenance false -t $CODE .
else
    echo "unsupported $ARCH"
    exit 1
fi

docker save -o $CODE-$VERSION.tar $CODE
md5sum $CODE-$VERSION.tar* >>$CODE-$VERSION.md5

echo "done($CODE-$VERSION.tar)."

exit 0
