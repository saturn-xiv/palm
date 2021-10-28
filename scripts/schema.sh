#!/bin/bash

set -e

# https://grpc.io/docs/languages/cpp/quickstart/
function build_grpc() {
    if [ -L $HOME/.local/bin/protoc ]
    then
        return
    fi
    if [ -d $HOME/downloads/grpc ]
    then
        cd $HOME/downloads/grpc
        git checkout $1
        git submodule update --init --recursive
    else
        git clone --recurse-submodules -b $1 https://github.com/grpc/grpc.git $HOME/downloads/grpc
    fi
    cd $HOME/downloads/grpc/third_party/protobuf
    git checkout $2
    if [ -d $HOME/build/grpc-amd64 ]
    then
        rm -r $HOME/build/grpc-amd64
    fi
    mkdir -pv $HOME/build/grpc-amd64
    cd $HOME/build/grpc-amd64
    cmake -DCMAKE_BUILD_TYPE=Release \
    -DgRPC_INSTALL=ON \
    -DgRPC_SSL_PROVIDER=package \
    -DgRPC_BUILD_TESTS=OFF \
    -DCMAKE_INSTALL_PREFIX=$HOME/.local $HOME/downloads/grpc
    make
    make install
}


declare -a languages=(
    # "node"
    "php"
    "python"
    "ruby"
    "cpp"
)

build_grpc "v1.39.1" "v3.17.1"
export PROTOBUF_ROOT=$HOME/.local
for l in "${languages[@]}"
do
    if [ -d clients/$l ]
    then
        rm -rfv clients/$l
    fi
    mkdir -pv clients/$l
    $PROTOBUF_ROOT/bin/protoc -I ./protos \
        -I $PROTOBUF_ROOT/include/google/protobuf \
        --${l}_out=clients/$l --grpc_out=clients/$l \
        --plugin=protoc-gen-grpc=$PROTOBUF_ROOT/bin/grpc_${l}_plugin \
        ./protos/*.proto
done

echo 'done.'
exit 0
