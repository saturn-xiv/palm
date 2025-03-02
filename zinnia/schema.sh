#!/bin/bash

set -e

export WORKDIR=$PWD

function generate_for_zinnia() {
    echo 'generate protocols for zinnia'

    cd $WORKDIR
    python -m grpc_tools.protoc \
        -I/usr/local/include \
        -Izinnia/protocols=$WORKDIR/protocols \
        --python_out=. --grpc_python_out=. \
        $WORKDIR/protocols/*.proto
}

generate_for_zinnia

echo 'done.'
