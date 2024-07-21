#!/bin/bash

set -e

. /etc/os-release

export SOURCE_ROOT=$HOME/downloads/grpc
export BUILD_ROOT=$HOME/build/grpc
export INSTALL_ROOT=$HOME/.local

function build_grpc() {
    # https://grpc.io/docs/languages/cpp/quickstart/
    if [ -L $HOME/.local/bin/protoc ]
    then
        echo 'already exists!'
        exit 0
    fi
    if [ -d $SOURCE_ROOT ]
    then
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

    cd $HOME/downloads/grpc/third_party/protobuf
    git checkout main
    git pull
    git checkout $2
    git submodule update --init --recursive

   
    if [ -d $BUILD_ROOT ]
    then
        rm -r $BUILD_ROOT
    fi
    mkdir -p $BUILD_ROOT
    cd $BUILD_ROOT
    cmake -DCMAKE_BUILD_TYPE=Release \
        -DABSL_PROPAGATE_CXX_STD=ON \
        -DgRPC_INSTALL=ON -DgRPC_SSL_PROVIDER=package -DgRPC_BUILD_TESTS=OFF \
        -DCMAKE_INSTALL_PREFIX=$INSTALL_ROOT $SOURCE_ROOT
    make -j $(nproc --ignore=2)
    make install

    echo "done($1, $2)."
}

# if [ "$#" -ne 2 ]
# then
#     echo "USAGE: $0 GRPC_VERSION PROTOBUF_VERSION"
#     exit 1
# fi

# vcpkg: 2023.04.15
build_grpc "v1.51.1" "v3.21.12"


if [[ $UBUNTU_CODENAME == "bionic" ]]
then
    cp $SOURCE_ROOT/third_party/re2/re2.pc $HOME/.local/lib/pkgconfig/
fi

exit 0
