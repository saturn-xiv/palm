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

generate_loquat

echo 'done.'
exit 0
