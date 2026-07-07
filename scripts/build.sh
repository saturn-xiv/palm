#!/bin/bash

set -e

source /etc/os-release

export WORK_DIR=$PWD
export TARGET_DIR=$WORK_DIR/tmp
export PACKAGE=palm-$VERSION_CODENAME-$(uname -m)-$(git describe --tags --always --dirty)

if [[ "$ID" != "ubuntu" ]]; then
    echo "Unsupported system $PRETTY_NAME"
    exit 1
fi

# ---------------------------------------------------------

function build_loquat() {
    sudo apt install -y libboost-all-dev

    local source_root=$WORK_DIR/loquat
    local build_root=$source_root/build/Release-$VERSION_CODENAME-$(uname -m)
    
    cmake -Wno-dev -DCMAKE_BUILD_TYPE=Release \
        -DABSL_PROPAGATE_CXX_STD=ON \
        -DTINK_USE_SYSTEM_OPENSSL=ON -DTINK_BUILD_TESTS=OFF \
        -B $build_root -S $source_root \
        -G Ninja
    cmake --build $build_root

    mkdir -p $TARGET_DIR/$PACKAGE/bin
    cp $build_root/loquat $TARGET_DIR/$PACKAGE/bin/
}

# https://phoenix.hexdocs.pm/releases.html
function build_rhododendron() {
    cd $WORK_DIR/rhododendron/

    mix deps.get --only prod
    MIX_ENV=prod mix clean

    MIX_ENV=prod mix compile
    MIX_ENV=prod mix assets.deploy
    MIX_ENV=prod mix phx.gen.release
    MIX_ENV=prod mix release --overwrite

    cp -r _build/prod/rel/rhododendron $TARGET_DIR/$PACKAGE/
}
# ---------------------------------------------------------

if [ -f $TARGET_DIR/$PACKAGE.md5 ]
then
    echo "release $PACKAGE already exists."
    exit 1
fi

if [ -f $TARGET_DIR/$PACKAGE.tar.xz ]
then
    rm $TARGET_DIR/$PACKAGE.tar.xz
fi

if [ -d $TARGET_DIR/$PACKAGE ]
then
    rm -r $TARGET_DIR/$PACKAGE
fi

mkdir -p $TARGET_DIR/$PACKAGE
# build_loquat
build_rhododendron

XZ_OPT=-9 tar -cJf $TARGET_DIR/$PACKAGE.tar.xz -C $TARGET_DIR/$PACKAGE .
md5sum $TARGET_DIR/$PACKAGE.tar.xz > $TARGET_DIR/$PACKAGE.md5

echo "done($PACKAGE)."
exit 0
