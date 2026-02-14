#!/bin/bash

set -e

export VERSION="$(uname -m)-$(date "+%4Y%m%d%H%M%S")"

if [ "$#" -ne 1 ]; then
    echo "USAGE: $0 FOLDER"
    exit 1
fi

if [ ! -d $1/node_modules ]
then
    echo "folder $1 not exists."
    exit 1
fi

XZ_OPT=-9 tar -cJf node_modules-$VERSION.tar.xz -C $1 node_modules package-lock.json
md5sum node_modules-$VERSION.tar.xz > node_modules-$VERSION.md5

echo "done($VERSION)."
exit 0
