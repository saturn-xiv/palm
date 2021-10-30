#!/bin/bash

set -e

export WORKSPACE=$PWD
export GIT_VERSION=$(git describe --tags --always --dirty --first-parent)
export CONAN_HOME=$WORKSPACE/docker/clang/conan

export CMAKE_LIBSERIAL_OPTIONS="-DINSTALL_SHARED=OFF \
    -DLIBSERIAL_BUILD_DOCS=OFF \
    -DLIBSERIAL_ENABLE_TESTING=OFF \
    -DLIBSERIAL_PYTHON_ENABLE=OFF \
    -DLIBSERIAL_BUILD_EXAMPLES=OFF \
    -DCASBIN_BUILD_TEST=OFF \
    -DCASBIN_BUILD_BENCHMARK=OFF \
    -DCASBIN_BUILD_BINDINGS=OFF \
    -DCASBIN_BUILD_PYTHON_BINDINGS=OFF \
    -DCASBIN_INSTALL=OFF"

build_backend() {
    echo "build $1@$2..."
    mkdir -pv $WORKSPACE/build/$1-$2
    cd $WORKSPACE/build/$1-$2
    conan install --build=missing --profile:build=default \
        --profile:host=$CONAN_HOME/profiles/$1 $CONAN_HOME
    cmake $WORKSPACE -DCMAKE_BUILD_TYPE=$2 \
        $CMAKE_LIBSERIAL_OPTIONS \
        -DCMAKE_TOOLCHAIN_FILE=$CONAN_HOME/toolchains/$1.cmake
    make
}

build_dashboard(){
    cd $WORKSPACE
    if [ ! -d node_modules ]
    then
        yarn install
    fi
    cd $WORKSPACE/dashboard
    if [ ! -d node_modules ]
    then
        yarn install
    fi
    # FIXME https://github.com/webpack/webpack/issues/14532
    NODE_OPTIONS=--openssl-legacy-provider yarn build
}

# -----------------------------------------------------------------------------

build_dashboard

build_backend amd64 Debug
build_backend amd64 Release
build_backend armhf Release
build_backend arm64 Release

echo 'done.'
exit 0
