#!/bin/bash

set -e

. /etc/os-release

export WORKSPACE=$PWD
export GIT_VERSION=$(git describe --tags --always --dirty --first-parent)
export PACKAGE_NAME="palm-$VERSION_CODENAME-$GIT_VERSION"
export TARGET_DIR=$WORKSPACE/tmp/$PACKAGE_NAME

# -----------------------------------------------------------------------------

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

function install_deb() {
    apt -y install libc6-dev:$1 libudev-dev:$1 libssl-dev:$1 \
        libpq5:$1 libpq-dev:$1 libmysqlclient-dev:$1 libsqlite3-dev:$1 libczmq-dev:$1
}

function build_go_project() {
    cd $WORKSPACE/$1

    local pkg="github.com/saturn-xiv/palm/$1/cmd"
    local ldflags="-s -w -X '$pkg.repo_url=$(git remote get-url origin)' -X '$pkg.author_name=$(git config --get user.name)' -X '$pkg.author_email=$(git config --get user.email)' -X '$pkg.build_time=$(date -u)' -X '$pkg.git_version=$(git describe --tags --always --dirty --first-parent)'"

    echo "build $1 for $3"
    GOOS=linux GOARCH=$2 go build -ldflags "$ldflags"
    mkdir -p $TARGET_DIR/bin/$3
    mv $1 $TARGET_DIR/bin/$3/
}

# -----------------------------------------------------------------------------

if [ $ID != "ubuntu" ]; then
    echo "unsupported system $ID"
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

build_go_project lilac amd64 x86_64
build_go_project lilac arm64 aarch64
build_go_project lilac riscv64 riscv64

install_deb amd64
build_cargo_x86_64 aloe
build_cargo_x86_64 fig

install_deb arm64
build_cargo_aarch64 aloe
build_cargo_aarch64 fig

build_musa
copy_jdk

build_morus
copy_nodejs

build_dashboard fig

copy_envoy
copy_assets

exit 0

# function build_cargo_musl() {
#     echo "build $2 for $1"
#     local target="$1-unknown-linux-musl"

#     cargo clean
#     cargo build --quiet --release --target $target -p $2

#     mkdir -p $WORKSPACE/bin/$1
#     cp target/$target/release/$2 $TARGET_DIR/bin/$1/
# }

# function build_lily() {
#     echo "build lily project"
#     cd $WORKSPACE
#     cp -a lily $TARGET_DIR/
# }

# function build_gardenia() {
#     echo "build gardenia project"
#     cd $WORKSPACE/gardenia/
#     mvn clean
#     mvn package -Dmaven.test.skip=true

#     mkdir -p $TARGET_DIR/gardenia
#     cp -r application-orig.properties \
#         target/gardenia-*.jar $TARGET_DIR/gardenia/

# }

# function build_loquat() {
#     echo "build loquat with gcc-$1"
#     apt install -y cmake g++-$1 golang libunwind-dev libboost-all-dev

#     local arch=$(uname -p)
#     local build_root=$HOME/build/loquat-$arch

#     mkdir -p $build_root
#     CC=gcc-$1 CXX=g++-$1 cmake -DCMAKE_BUILD_TYPE=Release \
#         -DABSL_PROPAGATE_CXX_STD=ON -DTINK_USE_SYSTEM_OPENSSL=ON \
#         -DBUILD_COMPILER=OFF -DWITH_OPENSSL=ON -DWITH_QT5=OFF -DBUILD_C_GLIB=OFF -DBUILD_JAVA=OFF -DBUILD_JAVASCRIPT=OFF -DBUILD_NODEJS=OFF -DBUILD_PYTHON=OFF \
#         -B $build_root -S $WORKSPACE/loquat
#     # -j $(nproc --ignore=2)
#     make -C $build_root

#     mkdir -p $TARGET_DIR/bin/$arch
#     cp $build_root/loquat $TARGET_DIR/bin/$arch/
# }

# function build_casbin_server() {
#     # https://pkg.go.dev/cmd/link
#     echo "build casbin-server@$2"

#     GOOS=linux GOARCH=$1 go build -o casbin -ldflags "-s -w"
#     mkdir -p $TARGET_DIR/bin/$2
#     mv casbin $TARGET_DIR/bin/$2/
# }

# # -----------------------------------------------------------------------------

# if [ -f $TARGET_DIR.tar.xz ]; then
#     echo "$PACKAGE_NAME.tar.xz already exists!"
#     exit 0
# fi

# if [ -d $TARGET_DIR ]; then
#     rm -r $TARGET_DIR
# fi
# mkdir -p $TARGET_DIR

# build_lily
# build_morus
# build_musa
# build_gardenia

# # ---------------------------
# cd $WORKSPACE/casbin-server/
# # diff -u casbin-server/proto/casbin.proto palm/protocols/casbin.proto >scripts/casbin/proto.patch
# patch proto/casbin.proto $WORKSPACE/scripts/casbin/proto.patch
# # git diff server > ../scripts/casbin/enforcer.patch
# git apply $WORKSPACE/scripts/casbin/enforcer.patch
# # https://github.com/casbin/casbin-server#protobuf-if-not-installed
# protoc -I proto \
#     --go_out=proto --go_opt=paths=source_relative \
#     --go-grpc_out=proto --go-grpc_opt=paths=source_relative \
#     proto/casbin.proto
# go get -u google.golang.org/protobuf
# go get -u google.golang.org/grpc
# go mod tidy
# build_casbin_server amd64 x86_64
# build_casbin_server arm64 aarch64
# build_casbin_server arm armhf
# build_casbin_server riscv64 riscv64
# mkdir -p $TARGET_DIR/casbin
# cp examples/rbac_model.conf $TARGET_DIR/casbin/
# cp config/connection_config_psql_example.json $TARGET_DIR/casbin/config-orig.json
# git checkout -f
# # ---------------------------

# if [ $UBUNTU_CODENAME == "jammy" ]; then
#     build_loquat 12
# fi

# if [ $UBUNTU_CODENAME == "focal" ]; then
#     build_loquat 11
# fi

# if [ $UBUNTU_CODENAME == "bionic" ]; then
#     build_loquat 11
# fi

# build_cargo_musl x86_64 coconut
# build_cargo_musl aarch64 coconut

# build_dashboard fig
# build_dashboard aloe

# build_cargo_x86_64

# install_deb arm64
# build_cargo_aarch64

# install_deb armhf
# build_cargo_armhf

# copy_assets
# copy_jdk
