#!/bin/bash

set -e

export PROTOBUF_ROOT=$HOME/.local
export WORKSPACE=$PWD

function generate_gourd() {
    echo "generate gourd project"
    if [ -d $WORKSPACE/gourd/include ]; then
        rm -r $WORKSPACE/gourd/include
    fi
    if [ -d $WORKSPACE/gourd/src ]; then
        rm -r $WORKSPACE/gourd/src
    fi

    mkdir -p $WORKSPACE/gourd/src
    cd $WORKSPACE/
    thrift -v -strict -out gourd/src --gen cpp:no_skeleton,include_prefix -r petunia/*.thrift
    mkdir -p gourd/include/petunia
    mv gourd/src/*.h gourd/include/petunia/
}

function generate_grpc_by_lang() {
    local target=$WORKSPACE/$2
    echo "generate grpc($1) => $2"
    $PROTOBUF_ROOT/bin/protoc -I $WORKSPACE/protocols \
        -I $PROTOBUF_ROOT/include/google/protobuf \
        --${1}_out=$target --grpc_out=$target \
        --plugin=protoc-gen-grpc=$PROTOBUF_ROOT/bin/grpc_${1}_plugin \
        $WORKSPACE/petunia/*.proto
}

function generate_lemon() {
    if [ -d $WORKSPACE/lemon/include ]; then
        rm -r $WORKSPACE/lemon/include
    fi
    if [ -d $WORKSPACE/lemon/src ]; then
        rm -r $WORKSPACE/lemon/src
    fi
    mkdir -p $WORKSPACE/lemon/src
    echo "generate lemon project"

    $PROTOBUF_ROOT/bin/protoc -I $WORKSPACE/petunia \
        -I $PROTOBUF_ROOT/include/google/protobuf \
        --cpp_out=$WORKSPACE/lemon/src --grpc_out=$WORKSPACE/lemon/src \
        --plugin=protoc-gen-grpc=$PROTOBUF_ROOT/bin/grpc_cpp_plugin \
        $WORKSPACE/petunia/*.proto

    mkdir -p $WORKSPACE/lemon/include
    mv $WORKSPACE/lemon/src/*.h $WORKSPACE/lemon/include/
}
# -----------------------------------------------------------------------------

generate_gourd
generate_lemon

echo 'done.'

exit 0
