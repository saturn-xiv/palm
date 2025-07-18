#!/bin/bash

set -e

export PROTOBUF_HOME=$HOME/.local

export WORK_DIR=$PWD
export PROTOCOLS_HOME=$WORK_DIR/protocols

function generate_gourd_thrift() {
    local target=$WORK_DIR/gourd

    echo "generate thrift protocols(cpp) for gourd..."
    rm -f $target/include/*.h $target/src/*.cpp
    thrift -out $target --gen cpp:no_skeleton -r $PROTOCOLS_HOME/*.thrift
    mv $target/*.h $target/include/
    mv $target/*.cpp $target/src/
}

function generate_gourd_grpc() {
    echo "generate grpc protocols(cpp) for gourd..."
    local target=$WORK_DIR/gourd

    $PROTOBUF_HOME/bin/protoc -I $PROTOCOLS_HOME \
        -I $PROTOBUF_HOME/include/google/protobuf \
        --cpp_out=$target --grpc_out=$target \
        --plugin=protoc-gen-grpc=$PROTOBUF_HOME/bin/grpc_cpp_plugin \
        $PROTOCOLS_HOME/*.proto
    mv $target/*.h $target/include/
    mv $target/*.cc $target/src/
}


function generate_phlox_dashboard() {
    echo "generate grpc protocols(typescript) for phlox..."
    local target=$WORK_DIR/phlox/dashboard/src/protocols
    if [ -d $target ]
    then
        rm -r $target
    fi
    mkdir -p $target
    $PROTOBUF_HOME/bin/protoc -I $PROTOCOLS_HOME \
        -I $PROTOBUF_HOME/include/google/protobuf \
        --js_out=import_style=commonjs,binary:$target \
        --grpc-web_out=import_style=typescript,mode=grpcweb:$target \
        portal.proto monitoring.proto
}

echo "clean gourd project"
if [ -d $WORK_DIR/gourd ]; then
    rm -r $WORK_DIR/gourd
fi
mkdir -p $WORK_DIR/gourd/include $WORK_DIR/gourd/src
generate_gourd_thrift
generate_gourd_grpc

generate_phlox_dashboard

echo 'done.'
exit 0
