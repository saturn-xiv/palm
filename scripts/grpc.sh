#!/bin/bash

set -e

. /etc/os-release

export SOURCE_ROOT=$HOME/downloads/grpc
export BUILD_ROOT=$HOME/build/grpc
export INSTALL_ROOT=$HOME/.local

# pip install cmake==3.31.6

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
        git submodule update --init --recursive
    else
        git clone --recurse-submodules -b $1 https://github.com/grpc/grpc.git $SOURCE_ROOT
    fi

    cd $SOURCE_ROOT/third_party/protobuf
    git checkout $2
    git submodule update --init --recursive

    if [ -d $BUILD_ROOT ]; then
        rm -r $BUILD_ROOT
    fi
    # https://github.com/abseil/abseil-cpp/pull/1536
    CC=/usr/lib/llvm18/bin/clang CXX=/usr/lib/llvm18/bin/clang++ cmake -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_STANDARD=17 \
        -DABSL_PROPAGATE_CXX_STD=ON \
        -DgRPC_INSTALL=ON -DgRPC_SSL_PROVIDER=package -DgRPC_BUILD_TESTS=OFF \
        -DCMAKE_INSTALL_PREFIX=$INSTALL_ROOT -B $BUILD_ROOT -S $SOURCE_ROOT \
        -G Ninja
    cmake --build $BUILD_ROOT
    cmake --install $BUILD_ROOT
}

# if [ "$#" -ne 2 ]; then
#     echo "USAGE: $0 $GRPC_VERSION PROTOBUF_VERSION"
#     exit 1
# fi

build_grpc v1.69.0 v31.1
echo "done."

exit 0
