#!/bin/bash

set -e

export PROTOBUF_ROOT=$HOME/.local
export WORKSPACE=$PWD

function generate_grpc_for_elixir() {
    echo "generate grpc-elixir $1 => $2"
    $PROTOBUF_ROOT/bin/protoc -I $WORKSPACE/protocols \
        -I $PROTOBUF_ROOT/include/google/protobuf \
        --elixir_out=$2/lib \
        $WORKSPACE/protocols/$1.proto
}

generate_grpc_for_elixir atropa tuberose

echo 'done.'
exit 0
