#!/bin/bash

set -e

export WORKSPACE=$PWD
export GIT_VERSION="$(date +"%Y.%m.%d")-$(git describe --tags --always --dirty --first-parent)"

cd $WORKSPACE/hyacinth/
clang-format -i -- src/*.c src/*.h
zig fmt build.zig src/*.zig
zig build -Dversion=$GIT_VERSION -Doptimize=ReleaseSafe --summary all

echo "done($GIT_VERSION)."
exit 0
