#!/bin/bash

set -e

export VERSION=$(date "+%4Y%m%d%H%M%S")
export CODE="palm-spring"

# podman pull archlinux:latest
# podman build -t $CODE .
# podman save --format=oci-archive -o $CODE-$VERSION.tar $CODE

# xz -z -C sha256 --best -T 1 $TAR.tar
# md5sum $TAR.tar.xz >>$TAR.md5
# split -d -b 1G $CODE-$VERSION.tar.xz $CODE-$VERSION.tar.xz.
# md5sum $CODE-$VERSION.tar.xz* >>$CODE-$VERSION.md5
# split -d -b 1G $CODE-$VERSION.tar $CODE-$VERSION.tar.


docker pull archlinux:latest
docker build --network host --platform=linux/amd64 --provenance false -t $CODE .
# docker save -o $CODE-$VERSION.tar $CODE
# md5sum $CODE-$VERSION.tar* >>$CODE-$VERSION.md5

# cat $CODE-$VERSION.tar.?? >$CODE-$VERSION.tar

echo "done($CODE-$VERSION)."

exit 0
