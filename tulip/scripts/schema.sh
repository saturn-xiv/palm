#!/bin/bash

set -e


export WORKSPACE=$PWD
export PROTOBUF_HOME=$HOME/.local

# -----------------------------------------------------------------------------

function generate_belladonna() {
    echo "generate for belladonna"
    local target=$WORKSPACE/belladonna
    if [ -d $target ]
    then
        rm -rf $target/include/*.h $target/src/*.cc
    fi
    mkdir -p $target/include $target/src

    $PROTOBUF_HOME/bin/protoc \
        -I $WORKSPACE/proto -I $PROTOBUF_HOME/include/google/protobuf \
        --cpp_out=$target --grpc_out=$target \
        --plugin=protoc-gen-grpc=$PROTOBUF_HOME/bin/grpc_cpp_plugin \
        $WORKSPACE/proto/*.proto
    thrift -out $target --gen cpp:no_skeleton -r $WORKSPACE/proto/*.thrift
    mv $target/*.h $target/include/
    mv $target/*.cc $target/*.cpp $target/src/
}

function generate_loquat() {
    echo "generate for loquat"
    local target=$WORKSPACE/loquat
    thrift -out $target/src --gen cpp:no_skeleton -r $WORKSPACE/proto/*.thrift
    mv $target/src/*.h $target/include/
}

# -----------------------------------------------------------------------------

generate_belladonna
generate_loquat

echo 'done.'
exit 0
