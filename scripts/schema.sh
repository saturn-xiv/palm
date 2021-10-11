#!/bin/bash

set -e

declare -a languages=(
    # "node"
    "php"
    "python"
    "ruby"
    "cpp"
)

# 1.37.0
export PROTOBUF_ROOT=$HOME/.local

for l in "${languages[@]}"
do
    if [ -d clients/$l ]
    then
        rm -rfv clients/$l
    fi
    mkdir -pv clients/$l
    $PROTOBUF_ROOT/bin/protoc -I protos \
        -I $PROTOBUF_ROOT/include/google/protobuf \
        --${l}_out=clients/$l --grpc_out=clients/$l \
        --plugin=protoc-gen-grpc=$PROTOBUF_ROOT/bin/grpc_${l}_plugin \
        protos/*.proto
done

echo 'done.'
exit 0
