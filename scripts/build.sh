#!/bin/bash

set -e

export WORKSPACE=$PWD
export GIT_VERSION="$(date +"%Y.%m.%d")-$(git describe --tags --always --dirty --first-parent)"

function build_zig() {
    cd $WORKSPACE/$1/    
    zig build -Dversion=$GIT_VERSION --release=safe --build-id=sha1 --summary all
    strip -s zig-out/bin/$1
}

clang-format -i -- begonia/src/*.c begonia/src/*.h
zig fmt hyacinth/build.zig hyacinth/src/*.zig begonia/build.zig begonia/src/*.zig

build_zig hyacinth

echo "done($GIT_VERSION)."
exit 0
