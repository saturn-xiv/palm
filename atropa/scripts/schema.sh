#!/bin/bash

set -e

export WORKSPACE=$PWD
export PROTOBUF_HOME=$HOME/.local

function generate_gourd() {
    echo "generate protocols(cpp) gourd"
    local target=$WORKSPACE/gourd
    if [ -d $target ]
    then
        rm -r $target
    fi
    mkdir -p $target/include $target/src

    $PROTOBUF_HOME/bin/protoc \
        -I $WORKSPACE/proto -I $PROTOBUF_HOME/include/google/protobuf \
        --cpp_out=$target --grpc_out=$target \
        --plugin=protoc-gen-grpc=$PROTOBUF_HOME/bin/grpc_cpp_plugin \
        $WORKSPACE/proto/*.proto
    mv $target/*.h $target/include/
    mv $target/*.cc $target/src/
}

# -----------------------------------------------------------------------------

generate_gourd

echo 'done.'
exit 0
