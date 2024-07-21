#!/bin/bash

set -e

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
        $WORKSPACE/protocols/*.proto
}

# -----------------------------------------------------------------------------

generate_gourd
generate_grpc_by_lang cpp lemon/src

echo 'done.'

exit 0
