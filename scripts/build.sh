#!/bin/bash

set -e

source /etc/os-release

export WORK_DIR=$PWD
export PACKAGE=palm-$VERSION_CODENAME-$(uname -m)-$(git describe --tags --always --dirty)
export TARGET_DIR=$WORK_DIR/tmp


# ---------------------------------------------------------

function build_wisteria() {
    cd $WORK_DIR/

    local target="$1-unknown-linux-gnu"
    echo "building wisteria for $target"
    cargo build --release --target $target

    mkdir -p $TARGET_DIR/$PACKAGE/bin/$1
    cp $WORK_DIR/target/$target/release/wisteria $TARGET_DIR/$PACKAGE/bin/$1/
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

declare -a targets=("x86_64" "aarch64" "riscv64gc")
for t in "${targets[@]}"; do
    build_wisteria $t
done

XZ_OPT=-9 tar -cJf $TARGET_DIR/$PACKAGE.tar.xz --remove-files -C $TARGET_DIR/$PACKAGE .
md5sum $TARGET_DIR/$PACKAGE.tar.xz > $TARGET_DIR/$PACKAGE.md5

echo "done($PACKAGE.tar.xz)."
exit 0
