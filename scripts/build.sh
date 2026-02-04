#!/bin/bash

set -e

export WORKSPACE=$PWD
export GIT_VERSION="$(date +"%Y.%m.%d")-$(git describe --tags --always --dirty --first-parent)"

function build_zig() {
    cd $WORKSPACE/$1/
    clang-format -i -- src/*.c src/*.h
    zig fmt build.zig src/*.zig
    zig build -Dversion=$GIT_VERSION --release=safe --build-id=sha1 --summary all
    strip -s zig-out/bin/$1
}

build_zig hyacinth

echo "done($GIT_VERSION)."
exit 0
