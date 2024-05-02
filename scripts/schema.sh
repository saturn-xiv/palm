#!/bin/bash

set -e

export PROTOBUF_ROOT=$HOME/.local
export WORKSPACE=$PWD

function generate_thrift_for_cpp() {
    cd $WORKSPACE

    echo "generate $1 => $2"
    if [ -d $2/src ]; then
        rm -r $2/src
    fi
    if [ -d $2/include ]; then
        rm -r $2/include
    fi

    mkdir -p $2/src
    thrift -out $2/src --gen cpp:no_skeleton -r $1
    mkdir -p $2/include
    mv $2/src/*.h $2/include/
}

generate_thrift_for_cpp $WORKSPACE/loquat/loquat.thrift $WORKSPACE/loquat/gourd

echo 'done.'
exit 0
