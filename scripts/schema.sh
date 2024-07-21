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

generate_gourd

echo 'done.'

exit 0
