#!/bin/bash

set -e

# ---------------------------------------------------------

declare -a languages=(
    # "node"
    "php"
    "python"
    "ruby"
    "cpp"
    "csharp"
    # "objective_c"
)


export WORKSPACE=$PWD
export PROTOBUF_ROOT=$HOME/.local

for l in "${languages[@]}"
do
    if [ -d $WORKSPACE/protocols/$l ]
    then
        rm -r $WORKSPACE/protocols/$l
    fi
    mkdir -pv $WORKSPACE/protocols/$l
    $PROTOBUF_ROOT/bin/protoc -I $WORKSPACE/protos \
        -I $PROTOBUF_ROOT/include/google/protobuf \
        --${l}_out=$WORKSPACE/protocols/$l --grpc_out=$WORKSPACE/protocols/$l \
        --plugin=protoc-gen-grpc=$PROTOBUF_ROOT/bin/grpc_${l}_plugin \
        $WORKSPACE/protos/*.proto
done

# ---------------------------------------------------------

if [ -d $WORKSPACE/models/src ]
then
    rm -f $WORKSPACE/models/src
fi
if [ -d $WORKSPACE/models/include ]
then
    rm -f $WORKSPACE/models/include
fi

cd $WORKSPACE/models
arc ../db/mappers/*.xml

# ---------------------------------------------------------
 
echo 'done.'
exit 0
