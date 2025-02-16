#!/bin/bash

set -e

. /etc/os-release

export SOURCE_ROOT=$HOME/downloads/grpc
export BUILD_ROOT=$HOME/build/grpc
export INSTALL_ROOT=$HOME/.local

function build_grpc() {
    # https://grpc.io/docs/languages/cpp/quickstart/
    if [ -L $HOME/.local/bin/protoc ]; then
        echo 'already exists!'
        exit 0
    fi
    if [ -d $SOURCE_ROOT ]; then
        cd $SOURCE_ROOT
        git checkout master
        git pull
        git checkout $1
        # fix unable to find current revision in submodule path
        # git pull --recurse-submodules
        git submodule update --init --recursive
    else
        git clone --recurse-submodules -b $1 https://github.com/grpc/grpc.git $SOURCE_ROOT
    fi

    if [ -d $BUILD_ROOT ]; then
        rm -r $BUILD_ROOT
    fi
    # https://github.com/abseil/abseil-cpp/pull/1536
    CC=clang CXX=clang++ cmake -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_STANDARD=17 \
        -DABSL_PROPAGATE_CXX_STD=ON \
        -DgRPC_INSTALL=ON -DgRPC_SSL_PROVIDER=package -DgRPC_BUILD_TESTS=OFF \
        -DCMAKE_INSTALL_PREFIX=$INSTALL_ROOT -B $BUILD_ROOT -S $SOURCE_ROOT
    make -j $(nproc --ignore=2) -C $BUILD_ROOT
    make install -C $BUILD_ROOT
}

if [ "$#" -ne 1 ]; then
    echo "USAGE: $0 GRPC_VERSION"
    exit 1
fi

build_grpc $1

echo "done($1)."
exit 0
