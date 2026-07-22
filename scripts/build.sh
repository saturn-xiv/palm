#!/bin/bash

set -e

source /etc/os-release

export WORK_DIR=$PWD
export PACKAGE=palm-$VERSION_CODENAME-$(uname -m)-$(git describe --tags --always --dirty)
export TARGET_DIR=$WORK_DIR/tmp


# ---------------------------------------------------------

function build_dashboard() {
    cd $WORK_DIR/$1/dashboard/
    if [ ! -d node_modules ]
    then
        npm install
    fi

    npm run build

    mkdir -p $TARGET_DIR/$PACKAGE/$1
    cp -r dist $TARGET_DIR/$PACKAGE/$1/dashboard
}

function build_wisteria_backend() {
    cd $WORK_DIR/

    local target="$1-unknown-linux-gnu"
    echo "building wisteria for $target"
    cargo build --release --target $target

    mkdir -p $TARGET_DIR/$PACKAGE/bin/$1
    cp $WORK_DIR/target/$target/release/wisteria $TARGET_DIR/$PACKAGE/bin/$1/
}

function build_wisteria_assets() {
    cd $WORK_DIR/wisteria/
    if [ ! -d node_modules ]
    then
        npm install
    fi

    local target=$TARGET_DIR/$PACKAGE/$1
    mkdir -p $target

    local -a items=(
        "@popperjs/core/dist/umd"
        "bootstrap/dist"
        "@tabler/core/dist"
        "@material/web"
        "bulma/css/bulma.min.css"
        "dayjs/dayjs.min.js"
        "dayjs/locale"
        "dayjs/plugin"
        "@fortawesome/fontawesome-free/css"
        "@fortawesome/fontawesome-free/js"
        "@fortawesome/fontawesome-free/sprites-full"
        "@fortawesome/fontawesome-free/svgs-full"
        "@fortawesome/fontawesome-free/webfonts"
        "@picocss/pico/css"
        "foundation-sites/dist"
    )
    for it in "${items[@]}"
    do
        local d=$(dirname $target/node_modules/$it)
        mkdir -p $d
        cp -r node_modules/$it $d/
    done

    cp -r db assets $target/
}

function build_marigold() {
    cd $WORK_DIR/marigold/
    mvn clean
    mvn package -Dmaven.test.skip=true

    mkdir -p $TARGET_DIR/$PACKAGE/marigold
    cp target/marigold-*.jar README.md $TARGET_DIR/$PACKAGE/marigold/
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
    build_wisteria_backend $t
done

build_dashboard wisteria
build_wisteria_assets
build_marigold

XZ_OPT=-9 tar -cJf $TARGET_DIR/$PACKAGE.tar.xz --remove-files -C $TARGET_DIR/$PACKAGE .
md5sum $TARGET_DIR/$PACKAGE.tar.xz > $TARGET_DIR/$PACKAGE.md5

echo "done($PACKAGE.tar.xz)."
exit 0
