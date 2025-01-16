#!/bin/bash

export VERSION=$(date "+%4Y%m%d%H%M%S")

function export_image() {
    local code="plone-$1"
    podman pull plone/$code:latest
    podman save --format=oci-archive -o $code-$VERSION.tar plone/$code
    md5sum $code-$VERSION.tar >>plone-$VERSION.md5
}

export_image backend
export_image frontend

echo "done(plone-$VERSION.tar)."

exit 0
