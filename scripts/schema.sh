#!/bin/bash

set -e

export PROTOBUF_HOME=$HOME/.local

export WORK_DIR=$PWD
export PROTOCOLS_HOME=$WORK_DIR/protocols

function generate_loquat() {
    local target=$WORK_DIR/loquat

    echo "generate thrift protocols for loquat"
    rm -f $target/include/*.h $target/src/*.cpp
    thrift -out $target --gen cpp:no_skeleton -r $PROTOCOLS_HOME/loquat.thrift
    mv $target/*.h $target/include/
    mv $target/*.cpp $target/src/
}

function generate_bamboo() {
    echo "generate bamboo protocols..."

    if [ -d $WORK_DIR/bamboo/include ]; then
        rm $WORK_DIR/bamboo/include/*.h
    fi
    if [ -d $WORK_DIR/bamboo/src ]; then
        rm -r $WORK_DIR/bamboo/src
    fi

    mkdir -p $WORK_DIR/bamboo/include $WORK_DIR/bamboo/src
    $PROTOBUF_HOME/bin/protoc -I $PROTOCOLS_HOME/protocols \
        -I $PROTOBUF_HOME/include/google/protobuf \
        --cpp_out=$WORK_DIR/bamboo/src --grpc_out=$WORK_DIR/bamboo/src \
        --plugin=protoc-gen-grpc=$PROTOBUF_ROOT/bin/grpc_cpp_plugin \
        $PROTOCOLS_HOME/casbin.proto
    mv $WORK_DIR/bamboo/src/*.h $WORK_DIR/bamboo/include/
}

generate_loquat
generate_bamboo

echo 'done.'
exit 0
