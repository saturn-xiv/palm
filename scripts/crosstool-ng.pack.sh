#!/bin/sh

set -e

export PACKAGE="x-tools-$(date +"%Y%m%d%H%M%S").tar.xz"

XZ_OPT=-9 tar cJf $PACKAGE x-tools

echo "done($PACKAGE)."
