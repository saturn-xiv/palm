#!/bin/bash

set -e

# https://github.com/casbin/casbin-cpp?tab=readme-ov-file#without-installing-casbin-locally
export CASBIN_FLAGS="-DCASBIN_INSTALL=OFF -DCASBIN_BUILD_TEST=OFF -DCASBIN_BUILD_BENCHMARK=OFF -DCASBIN_BUILD_BINDINGS=OFF -DCASBIN_BUILD_PYTHON_BINDINGS=OFF"
export THRIFT_FLAGS="-DCMAKE_BUILD_TYPE=Release -DBUILD_COMPILER=OFF -DWITH_OPENSSL=ON -DBUILD_JAVA=OFF -DBUILD_JAVASCRIPT=OFF -DBUILD_NODEJS=OFF -DBUILD_PYTHON=OFF"
# -DBOOST_DATE_TIME_POSIX_TIME_STD_CONFIG
export BOOST_FLAGS=""
export WORK_DIR=$PWD

function build_on_arch() {
    cd $WORK_DIR/

    # cmake --preset=arch -DVCPKG_TARGET_TRIPLET=x64-linux-release -DVCPKG_CHAINLOAD_TOOLCHAIN_FILE=$WORK_DIR/toolchains/arch/clang.cmake $BOOST_FLAGS $THRIFT_FLAGS $CASBIN_FLAGS
    # cmake --build $WORK_DIR/build/arch

    cmake --preset=arch -DVCPKG_TARGET_TRIPLET=x64-linux-release $BOOST_FLAGS $THRIFT_FLAGS $CASBIN_FLAGS
    cmake --build $WORK_DIR/build/arch
}

function build_x86_64_on_ubuntu() {
    cd $WORK_DIR/
    cmake --preset=x86_64 -DVCPKG_TARGET_TRIPLET=x64-linux-release $BOOST_FLAGS $THRIFT_FLAGS $CASBIN_FLAGS
    cmake --build $WORK_DIR/build/x86_64

    # cmake --preset=x86_64 -DVCPKG_TARGET_TRIPLET=x64-linux-release -DVCPKG_CHAINLOAD_TOOLCHAIN_FILE=$WORK_DIR/toolchains/ubuntu/clang/x86_64.cmake $THRIFT_FLAGS
    # cmake --build $WORK_DIR/build/x86_64

    build_deb x86_64 amd64
}

function build_aarch64_on_ubuntu() {
    cd $WORK_DIR/
    cmake --preset=aarch64 -DVCPKG_TARGET_TRIPLET=arm64-linux-release -DVCPKG_CHAINLOAD_TOOLCHAIN_FILE=$WORK_DIR/toolchains/ubuntu/gcc/aarch64.cmake $BOOST_FLAGS $THRIFT_FLAGS $CASBIN_FLAGS
    cmake --build $WORK_DIR/build/aarch64

    build_deb aarch64 arm64
}

function build_deb() {
    local target=$WORK_DIR/tmp/$VERSION_CODENAME-$1/palm
    local version=$(date +"%Y.%-m-%-d")
    if [ -d $target ]; then
        rm -rf $target
    fi
    mkdir -p $target/DEBIAN
    cat >$target/DEBIAN/control <<EOF
Package: palm
Version: $version
Maintainer: Jeremy
Architecture: $2
Description: A total free education and translation solution
Homepage: https://github.com/saturn-xiv/palm
EOF

    mkdir -p $target/etc/palm

    mkdir -p $target/usr/bin
    cd $WORK_DIR/build/$1/
    cp -v lavender/lavender bamboo/bamboo phlox/phlox $target/usr/bin/

    mkdir -p $target/usr/share/palm/
    cd $WORK_DIR
    cp -rv README.md LICENSE $target/usr/share/palm/

    mkdir -p $target/var/lib/palm/lavender
    cd $WORK_DIR/lavender/
    cp -rv README.md assets views locales vendors $target/var/lib/palm/lavender/ 
    mkdir -p $target/var/lib/palm/lavender/db
    cd $WORK_DIR/lavender/db
    cp -rv README.md migrations $target/var/lib/palm/lavender/db/

    mkdir -p $target/var/lib/palm/bamboo $target/usr/share/palm/bamboo 
    cd $WORK_DIR/bamboo/
    cp -rv README.md $target/usr/share/palm/bamboo/

    mkdir -p $target/var/lib/palm/phlox $target/usr/share/palm/phlox 
    cd $WORK_DIR/phlox/
    cp -rv README.md $target/usr/share/palm/phlox/    
    build_dashboard $WORK_DIR/phlox/dashboard $target/usr/share/palm/phlox/dashboard

    build_assets $target

    cd $(dirname $target)
    # dpkg-deb -x xxx.deb xxx
    # dpkg-deb -I xxx.deb
    dpkg-deb -b palm palm-$2-$version-$VERSION_CODENAME.deb
}

function build_dashboard() {
    echo "build dashboard($1)"
    cd $WORK_DIR/$1
    if [ ! -d node_modules ]; then
        npm install
    fi
    npm run build
    cp -rv dist $2
}

function build_assets() {
    cd $WORK_DIR/
    if [ ! -d node_modules ]; then
        npm install
    fi

    local -a assets=(
        "bootstrap/dist"
        "bulma/css/bulma.min.css"
        "marked/lib/marked.umd.js"
    )
    for i in "${assets[@]}"; do
        mkdir -p $target/usr/share/palm/node_modules/$(dirname $i)
        cp -rv node_modules/$i $target/usr/share/palm/node_modules/$(dirname $i)/
    done
}

. /etc/os-release

if [ $ID == "arch" ]; then
    build_on_arch
elif [ $ID == "ubuntu" ]; then
    build_x86_64_on_ubuntu
    build_aarch64_on_ubuntu
else
    echo "unsupported os $PRETTY_NAME"
fi

echo "done($PRETTY_NAME)"
exit 0
