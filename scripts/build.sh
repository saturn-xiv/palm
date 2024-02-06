#!/bin/bash

set -e

. /etc/os-release

export WORKSPACE=$PWD
export GIT_VERSION=$(git describe --tags --always --dirty --first-parent)
export PACKAGE_NAME="palm-$VERSION_CODENAME-$GIT_VERSION"
export TARGET_DIR=$WORKSPACE/tmp/$PACKAGE_NAME

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
    mv dist $TARGET_DIR/$1/dashboard
}

function install_deb() {
    apt -y install libc6-dev:$1 libudev-dev:$1 libssl-dev:$1 \
        libpq5:$1 libpq-dev:$1 libmysqlclient-dev:$1 libsqlite3-dev:$1
}

function build_cargo_musl() {
    echo "build $2 for $1"
    local target="$1-unknown-linux-musl"

    cargo clean
    cargo build --quiet --release --target $target -p $2

    mkdir -p $WORKSPACE/bin/$1
    cp target/$target/release/$2 $TARGET_DIR/bin/$1/
}

function build_cargo_x86_64() {
    echo "build palm for x84-64"
    cd $WORKSPACE

    local CC=gcc
    local CXX=g++

    local PKG_CONFIG_ALL_STATIC=1

    local target="x86_64-unknown-linux-gnu"

    cargo clean
    cargo build --release --target $target
    cd target/$target/release/
    mkdir -p $TARGET_DIR/bin/x86_64/
    cp aloe fig $TARGET_DIR/bin/x86_64/
}

function build_cargo_aarch64() {
    echo "build palm for aarch64"
    cd $WORKSPACE

    local CC=aarch64-linux-gnu-gcc
    local CXX=aarch64-linux-gnu-g++
    local HOST_CC=gcc

    local PKG_CONFIG_ALLOW_CROSS=1
    local PKG_CONFIG_ALL_STATIC=1
    local PKG_CONFIG_DIR=
    local PKG_CONFIG_LIBDIR=/usr/lib/aarch64-linux-gnu/pkgconfig

    local target="aarch64-unknown-linux-gnu"

    cargo clean
    cargo build --release --target $target
    cd target/$target/release/
    mkdir -p $TARGET_DIR/bin/aarch64/
    cp aloe fig $TARGET_DIR/bin/aarch64/
}

function build_cargo_armhf() {
    echo "build palm for armhf"
    cd $WORKSPACE

    local CC=arm-linux-gnueabihf-gcc
    local CXX=arm-linux-gnueabihf-g++
    local HOST_CC=gcc

    local PKG_CONFIG_ALLOW_CROSS=1
    local PKG_CONFIG_ALL_STATIC=1
    local PKG_CONFIG_DIR=
    local PKG_CONFIG_LIBDIR=/usr/lib/arm-linux-gnueabihf/pkgconfig

    local target="armv7-unknown-linux-gnueabihf"

    cargo clean
    cargo build --release --target $target
    cd target/$target/release/
    mkdir -p $TARGET_DIR/bin/armhf/
    cp aloe fig $TARGET_DIR/bin/armhf/
}

copy_assets() {
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

    echo "copy palm assets"
    cd $WORKSPACE
    cp -a README.md LICENSE \
        $TARGET_DIR/

    echo "$GIT_VERSION" >$TARGET_DIR/VERSION
    echo "$(date -R)" >>$TARGET_DIR/VERSION
}

function build_lily() {
    echo "build lily project"
    cd $WORKSPACE
    cp -a lily $TARGET_DIR/
}

function build_morus() {
    echo "build morus project"
    cd $WORKSPACE/morus/
    if [ ! -d node_modules ]; then
        npm install
    fi
    if [ -d dist ]; then
        rm -r dist
    fi
    npx webpack --mode=production

    mv dist $TARGET_DIR/morus
    cp README.md config.json.orig $TARGET_DIR/morus/
}

function build_musa() {
    echo "build musa project"
    cd $WORKSPACE/musa/
    mvn clean
    mvn package -Dmaven.test.skip=true

    mkdir -p $TARGET_DIR/musa
    cp -r README.md application-orig.properties mybatis-config.xml com wechatpay-orig \
        target/musa-*.jar $TARGET_DIR/musa/

}

function build_gardenia() {
    echo "build gardenia project"
    cd $WORKSPACE/gardenia/
    mvn clean
    mvn package -Dmaven.test.skip=true

    mkdir -p $TARGET_DIR/gardenia
    cp -r application-orig.properties \
        target/gardenia-*.jar $TARGET_DIR/gardenia/

}

function build_loquat() {
    echo "build loquat with gcc-$1"
    apt install -y cmake g++-$1 golang libunwind-dev libboost-all-dev

    local arch=$(uname -p)
    local build_root=$HOME/build/loquat-$arch

    mkdir -p $build_root
    CC=gcc-$1 CXX=g++-$1 cmake -DCMAKE_BUILD_TYPE=Release \
        -DABSL_PROPAGATE_CXX_STD=ON -DTINK_USE_SYSTEM_OPENSSL=ON \
        -DBUILD_COMPILER=OFF -DWITH_OPENSSL=ON -DWITH_QT5=OFF -DBUILD_C_GLIB=OFF -DBUILD_JAVA=OFF -DBUILD_JAVASCRIPT=OFF -DBUILD_NODEJS=OFF -DBUILD_PYTHON=OFF \
        -B $build_root -S $WORKSPACE/loquat
    # -j $(nproc --ignore=2)
    make -C $build_root

    mkdir -p $TARGET_DIR/bin/$arch
    cp $build_root/loquat $TARGET_DIR/bin/$arch/
}

function build_casbin_server() {
    # https://pkg.go.dev/cmd/link
    echo "build casbin-server@$2"

    GOOS=linux GOARCH=$1 go build -o casbin -ldflags "-s -w"
    mkdir -p $TARGET_DIR/bin/$2
    mv casbin $TARGET_DIR/bin/$2/
}

function copy_jdk() {
    local x64_jdk_url="https://download.java.net/java/GA/jdk21.0.1/415e3f918a1f4062a0074a2794853d0d/12/GPL/openjdk-21.0.1_linux-x64_bin.tar.gz"
    local aarch64_jdk_url="https://download.java.net/java/GA/jdk21.0.1/415e3f918a1f4062a0074a2794853d0d/12/GPL/openjdk-21.0.1_linux-aarch64_bin.tar.gz"

    local jdk_version="21.0.1"
    local x64_jdk_file=$HOME/downloads/openjdk-${jdk_version}_linux-x64_bin.tar.gz
    local aarch64_jdk_file=$HOME/downloads/openjdk-${jdk_version}_linux-aarch64_bin.tar.gz

    if [ ! -f $x64_jdk_file ]; then
        wget -P $HOME/downloads/ $x64_jdk_url
    fi
    if [ ! -f $aarch64_jdk_file ]; then
        wget -P $HOME/downloads/ $aarch64_jdk_url
    fi

    mkdir -p $TARGET_DIR/jdk
    cd $TARGET_DIR/jdk/
    tar xf $x64_jdk_file
    mv jdk-$jdk_version x64
    tar xf $aarch64_jdk_file
    mv jdk-$jdk_version aarch64
}

# -----------------------------------------------------------------------------
if [ $ID != "ubuntu" ]; then
    echo "unsupported system $ID"
    exit 1
fi

if [ -f $TARGET_DIR.tar.xz ]; then
    echo "$PACKAGE_NAME.tar.xz already exists!"
    exit 0
fi

if [ -d $TARGET_DIR ]; then
    rm -r $TARGET_DIR
fi
mkdir -p $TARGET_DIR

build_lily
build_morus
build_musa
build_gardenia

# ---------------------------
cd $WORKSPACE/casbin-server/
# diff -u casbin-server/proto/casbin.proto palm/protocols/casbin.proto >scripts/casbin/proto.patch
patch proto/casbin.proto $WORKSPACE/scripts/casbin/proto.patch
# git diff server > ../scripts/casbin/enforcer.patch
git apply $WORKSPACE/scripts/casbin/enforcer.patch
# https://github.com/casbin/casbin-server#protobuf-if-not-installed
protoc -I proto \
    --go_out=proto --go_opt=paths=source_relative \
    --go-grpc_out=proto --go-grpc_opt=paths=source_relative \
    proto/casbin.proto
go get -u google.golang.org/protobuf
go get -u google.golang.org/grpc
go mod tidy
build_casbin_server amd64 x86_64
build_casbin_server arm64 aarch64
build_casbin_server arm armhf
build_casbin_server riscv64 riscv64
mkdir -p $TARGET_DIR/casbin
cp examples/rbac_model.conf $TARGET_DIR/casbin/
cp config/connection_config_psql_example.json $TARGET_DIR/casbin/config-orig.json
git checkout -f
# ---------------------------

if [ $UBUNTU_CODENAME == "jammy" ]; then
    build_loquat 12
fi

if [ $UBUNTU_CODENAME == "focal" ]; then
    build_loquat 11
fi

if [ $UBUNTU_CODENAME == "bionic" ]; then
    build_loquat 11
fi

build_cargo_musl x86_64 coconut
build_cargo_musl aarch64 coconut

build_dashboard fig
build_dashboard aloe

install_deb amd64
build_cargo_x86_64

install_deb arm64
build_cargo_aarch64

install_deb armhf
build_cargo_armhf

copy_assets
copy_jdk

cd $(dirname $TARGET_DIR)
XZ_OPT=-9 tar -cJf $PACKAGE_NAME.tar.xz $PACKAGE_NAME

echo "done($GIT_VERSION)."
exit 0
