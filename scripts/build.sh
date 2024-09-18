#!/bin/bash

set -e

. /etc/os-release

# ---------------------------------------------------------

export WORKSPACE=$PWD
export GIT_VERSION=$(git describe --tags --always --dirty --first-parent)
export PACKAGE_NAME="palm-$VERSION_CODENAME-$GIT_VERSION"
export TARGET_DIR=$WORKSPACE/tmp/$PACKAGE_NAME

function build_go() {
    cd $WORKSPACE/$1

    local pkg="github.com/saturn-xiv/palm/$1/cmd"
    local ldflags="-s -w -X '$pkg.repo_url=$(git remote get-url origin)' -X '$pkg.author_name=$(git config --get user.name)' -X '$pkg.author_email=$(git config --get user.email)' -X '$pkg.build_time=$(date -u)' -X '$pkg.git_version=$(git describe --tags --always --dirty --first-parent)'"

    echo "build $1 for $2"
    mkdir -p $TARGET_DIR/$2/bin
    GOOS=linux GOARCH=$2 go build -ldflags "$ldflags" -o $TARGET_DIR/$2/bin/$1
}

function build_rust_x86_64() {
    echo "build $1 for x84-64"
    cd $WORKSPACE

    local CC=gcc
    local CXX=g++
    local target="x86_64-unknown-linux-gnu"

    cargo build --release --target $target -p $1
    mkdir -p $TARGET_DIR/amd64/bin
    cp target/$target/release/$1 $TARGET_DIR/amd64/bin/
}

function build_rust_aarch64() {
    echo "build $1 for aarch64"
    cd $WORKSPACE

    local CC=aarch64-linux-gnu-gcc
    local CXX=aarch64-linux-gnu-g++
    local HOST_CC=gcc

    local PKG_CONFIG_ALLOW_CROSS=1
    local PKG_CONFIG_ALL_STATIC=1
    local PKG_CONFIG_DIR=
    local PKG_CONFIG_LIBDIR=/usr/lib/aarch64-linux-gnu/pkgconfig

    local target="aarch64-unknown-linux-gnu"

    cargo build --release --target $target -p $1
    mkdir -p $TARGET_DIR/arm64/bin
    cp target/$target/release/$1 $TARGET_DIR/arm64/bin/
}

function build_rust_armhf() {
    echo "build $1 for armhf"
    cd $WORKSPACE

    local CC=arm-linux-gnueabihf-gcc
    local CXX=arm-linux-gnueabihf-g++
    local HOST_CC=gcc

    local PKG_CONFIG_ALLOW_CROSS=1
    local PKG_CONFIG_ALL_STATIC=1
    local PKG_CONFIG_DIR=
    local PKG_CONFIG_LIBDIR=/usr/lib/arm-linux-gnueabihf/pkgconfig

    local target="armv7-unknown-linux-gnueabihf"

    cargo build --release --target $target -p $1
    mkdir -p $TARGET_DIR/armhf/bin
    cp target/$target/release/$1 $TARGET_DIR/armhf/bin/
}

function build_dashboard() {
    cd $WORKSPACE/$1/dashboard
    if [ ! -d node_modules ]; then
        npm install
    fi
    if [ -d dist ]; then
        rm -r dist
    fi
    npm run build

    mkdir -p $TARGET_DIR/$1
    cp -r dist $TARGET_DIR/$1/dashboard
}

function copy_jdk() {
    echo "install jdk..."
    local jdk_version="22.0.2"

    local x64_url="https://download.java.net/java/GA/jdk22.0.2/c9ecb94cd31b495da20a27d4581645e8/9/GPL/openjdk-22.0.2_linux-x64_bin.tar.gz"
    local aarch64_url="https://download.java.net/java/GA/jdk22.0.2/c9ecb94cd31b495da20a27d4581645e8/9/GPL/openjdk-22.0.2_linux-aarch64_bin.tar.gz"

    local x64_file=$HOME/downloads/openjdk-${jdk_version}_linux-x64_bin.tar.gz
    local aarch64_file=$HOME/downloads/openjdk-${jdk_version}_linux-aarch64_bin.tar.gz

    if [ ! -f $x64_file ]; then
        wget -P $HOME/downloads/ $x64_url
    fi
    if [ ! -f $aarch64_file ]; then
        wget -P $HOME/downloads/ $aarch64_url
    fi

    cd $TARGET_DIR/amd64/
    tar xf $x64_file
    cd $TARGET_DIR/arm64/
    tar xf $aarch64_file
}

# https://nodejs.org/en/download/prebuilt-binaries
function copy_nodejs() {
    echo "install nodejs..."
    local node_version="20.17.0"
    local x64_url="https://nodejs.org/dist/v${node_version}/node-v${node_version}-linux-x64.tar.xz"
    local arm64_url="https://nodejs.org/dist/v${node_version}/node-v${node_version}-linux-arm64.tar.xz"

    local x64_file=$HOME/downloads/node-v${node_version}-linux-x64.tar.xz
    local arm64_file=$HOME/downloads/node-v${node_version}-linux-arm64.tar.xz

    if [ ! -f $x64_file ]; then
        wget -P $HOME/downloads/ $x64_url
    fi
    if [ ! -f $arm64_file ]; then
        wget -P $HOME/downloads/ $arm64_url
    fi

    cd $TARGET_DIR/amd64/
    tar xf $x64_file
    cd $TARGET_DIR/arm64/
    tar xf $arm64_file

}

copy_fig_assets() {
    echo "copy fig assets"
    cd $WORKSPACE/fig/
    if [ ! -d node_modules ]; then
        npm install
    fi
    local -a packages=(
        "bootstrap/dist"
        "bulma/css"
        "marked/marked.min.js"
        "material-design-icons/iconfont"
        "d3/dist"
        "@fontsource/roboto"
        "moment/dist"
        "moment-timezone/builds/moment-timezone-with-data.min.js"
        "@popperjs/core/dist"
        "mdb-ui-kit/css"
        "mdb-ui-kit/js"
        "mdb-ui-kit/img"
        "qrcodejs/qrcode.min.js"
        "@fortawesome/fontawesome-free/js"
        "@fortawesome/fontawesome-free/css"
        "@fortawesome/fontawesome-free/svgs"
        "@fortawesome/fontawesome-free/webfonts"
        "@fortawesome/fontawesome-free/sprites"
        "famfamfam-flags/dist"
        "famfamfam-silk/dist"
        "famfamfam-mini/dist"
    )
    for i in "${packages[@]}"; do
        local p=node_modules/$i
        local t=$(dirname "$TARGET_DIR/fig/$p")
        mkdir -p $t
        cp -a $p $t/
    done

    cp -r db $TARGET_DIR/fig/
}

# https://github.com/envoyproxy/envoy/tags
function copy_envoy() {
    echo "install envoy..."
    local envoy_version="1.31.1"
    local x86_64_url="https://github.com/envoyproxy/envoy/releases/download/v${envoy_version}/envoy-${envoy_version}-linux-x86_64"
    local aarch64_url="https://github.com/envoyproxy/envoy/releases/download/v${envoy_version}/envoy-${envoy_version}-linux-aarch_64"

    local x86_64_file=$HOME/downloads/envoy-${envoy_version}-linux-x86_64
    local aarch64_file=$HOME/downloads/envoy-${envoy_version}-linux-aarch_64

    if [ ! -f $x86_64_file ]; then
        wget -P $HOME/downloads/ $x86_64_url
    fi
    if [ ! -f $aarch64_file ]; then
        wget -P $HOME/downloads/ $aarch64_url
    fi

    cp $x86_64_file $TARGET_DIR/amd64/bin/envoy
    chmod +x $TARGET_DIR/amd64/bin/envoy

    cp $aarch64_file $TARGET_DIR/arm64/bin/envoy
    chmod +x $TARGET_DIR/arm64/bin/envoy
}

# ---------------------------------------------------------

if [[ "$ID" != "ubuntu" ]]; then
    echo "Unsupported system: $ID"
    exit 1
fi

if [ -f ${TARGET_DIR}.tar.xz ]; then
    echo "check passed(${TARGET_DIR}.tar.xz)."
    exit 0
fi

if [ -d $TARGET_DIR ]; then
    rm -r $TARGET_DIR
fi
mkdir -p $TARGET_DIR

# ---------------------------------------------------------

# https://go.dev/src/internal/goarch/goarch.go
declare -a go_targets=(
    "amd64"
    "arm64"
    "riscv64"
    # "loong64"
)

for t in "${go_targets[@]}"; do
    build_go atropa $t
done

# ---------------------------------------------------------

declare -a rust_projects=(
    "fig"
    "camelia"
)

for p in "${rust_projects[@]}"; do
    build_dashboard $p
    build_rust_x86_64 $p
    build_rust_aarch64 $p
done

copy_fig_assets

# ---------------------------------------------------------

copy_jdk
copy_nodejs

echo "$GIT_VERSION" >$TARGET_DIR/VERSION
echo "$(date -R)" >>$TARGET_DIR/VERSION

# ---------------------------------------------------------

cd $(dirname $TARGET_DIR)
echo "compressing $PACKAGE_NAME..."
XZ_OPT=-9 tar -cJf $PACKAGE_NAME.tar.xz $PACKAGE_NAME
md5sum $PACKAGE_NAME.tar.xz >$PACKAGE_NAME.md5
echo "done($GIT_VERSION)."

exit 0
