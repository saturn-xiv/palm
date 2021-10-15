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
# export PROTOBUF_ROOT=$HOME/.local
export PROTOBUF_ROOT=$HOME/.conan/data/protobuf/3.17.1/_/_/package/64504d4b5743a18b5bb012ba0145fd09ce3bd5f2
export GRPC_ROOT=$HOME/.conan/data/grpc/1.39.1/_/_/package/8835978170244bbc1a72106955c26b2c147925a1

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
        --plugin=protoc-gen-grpc=$GRPC_ROOT/bin/grpc_${l}_plugin \
        protos/*.proto
done

echo 'done.'
exit 0
