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

function generate_thrift_for_go() {
    cd $WORKSPACE
    echo "generate $1 => $2"

    if [ -d $2 ]; then
        rm -r $2
    fi

    mkdir -p $2
    thrift -out $(dirname $2) --gen go:skip_remote,package=v1 -r $1
}

generate_thrift_for_cpp $WORKSPACE/loquat/loquat.thrift $WORKSPACE/loquat/gourd
generate_thrift_for_go $WORKSPACE/gourd/gourd.thrift $WORKSPACE/gourd/services/v1

echo 'done.'
exit 0
