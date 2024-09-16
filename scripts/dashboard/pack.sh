#!/bin/bash

set -e

export VERSION=$(date "+%4Y%m%d%H%M%S")

XZ_OPT=-9 tar -cJf dashboard-$VERSION.tar.xz node_modules package-lock.json

echo "done($VERSION)."

exit 0
