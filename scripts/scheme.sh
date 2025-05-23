#!/bin/bash

set -e

export PROTOBUF_HOME=$HOME/.local
export WORK_DIR=$PWD

function generate_loquat() {
    echo "generate protocols for loquat"
    local target=$WORK_DIR/loquat
    rm -f $target/include/*.h $target/src/*.cpp
    thrift -out $target --gen cpp:no_skeleton -r $target/loquat.thrift
    mv $target/*.h $target/include/
    mv $target/*.cpp $target/src/
}

function generate_phlox() {
    echo "generate protocols for phlox"

    local target=$WORK_DIR/phlox
    thrift -out $target --gen rs -r $WORK_DIR/loquat/loquat.thrift
    mv $target/loquat.rs $target/src/loquat/v1.rs
}

generate_loquat
generate_phlox

cargo fmt

echo 'done.'
exit 0
