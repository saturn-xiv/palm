#!/bin/sh

set -e

export PACKAGE="node_modules-$(date +"%Y%m%d%H%M%S").tar.xz"

XZ_OPT=-9 tar cJf $PACKAGE node_modules package.json package-lock.json

echo "done($PACKAGE)."
