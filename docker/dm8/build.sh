#!/bin/bash

set -e

export VERSION=$(date "+%4Y%m%d%H%M%S")
export CODE="palm-dm8"

buildah pull centos:7
buildah bud --layers -t $CODE .
podman save --format=oci-archive -o $CODE-$VERSION.tar $CODE
md5sum $CODE-$VERSION.tar >$CODE-$VERSION.md5

echo "done($CODE-$VERSION.tar)."

exit 0
