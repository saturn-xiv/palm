#!/bin/bash

set -e




declare -a languages=(
    # "node"
    "php"
    "python"
    "ruby"
    "cpp"
)


export WORKSPACE=$PWD
export PROTOBUF_ROOT=$HOME/.local

for l in "${languages[@]}"
do
    if [ -d $WORKSPACE/clients/$l ]
    then
        rm -rfv $WORKSPACE/clients/$l
    fi
    mkdir -pv $WORKSPACE/clients/$l
    $PROTOBUF_ROOT/bin/protoc -I $WORKSPACE/protos \
        -I $PROTOBUF_ROOT/include/google/protobuf \
        --${l}_out=$WORKSPACE/clients/$l --grpc_out=$WORKSPACE/clients/$l \
        --plugin=protoc-gen-grpc=$PROTOBUF_ROOT/bin/grpc_${l}_plugin \
        $WORKSPACE/protos/*.proto
done

echo 'done.'
exit 0
