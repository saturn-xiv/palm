#!/bin/bash

set -e

export PROTOBUF_ROOT=$HOME/.local
export WORKSPACE=$PWD

function generate_grpc_for_go() {
    echo "generate grpc $1 => $2"

    if [ -d $2 ]; then
        rm $2/*.pb.go
    else
        mkdir -p $2
    fi

    if [ ! -f $2/mod.go ]; then
        echo "package v2" >$2/mod.go
    fi

    $PROTOBUF_ROOT/bin/protoc -I $WORKSPACE/protocols \
        -I $PROTOBUF_ROOT/include/google/protobuf \
        --go_out=$2 --go_opt=paths=source_relative \
        --go-grpc_out=$2 --go-grpc_opt=paths=source_relative \
        $WORKSPACE/protocols/$1.proto
}

function generate_grpc_for_elixir() {
    echo "generate grpc-elixir $1 => $2"
    if [ -f $2/$1.pb.ex ]; then
        rm $2/$1.pb.ex
    fi
    $PROTOBUF_ROOT/bin/protoc -I $WORKSPACE/protocols \
        -I $PROTOBUF_ROOT/include/google/protobuf \
        --elixir_out=$2 \
        $WORKSPACE/protocols/$1.proto
}

generate_grpc_for_elixir atropa tuberose/lib
generate_grpc_for_go atropa atropa/services/v2

echo 'done.'
exit 0
