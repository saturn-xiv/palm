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
    echo "generate code for $l"
    if [ -d $WORKSPACE/protocols/$l ]
    then
        rm -r $WORKSPACE/protocols/$l
    fi
    mkdir -p $WORKSPACE/protocols/$l
    $PROTOBUF_ROOT/bin/protoc -I $WORKSPACE/protos \
        -I $PROTOBUF_ROOT/include/google/protobuf \
        --${l}_out=$WORKSPACE/protocols/$l --grpc_out=$WORKSPACE/protocols/$l \
        --plugin=protoc-gen-grpc=$PROTOBUF_ROOT/bin/grpc_${l}_plugin \
        $WORKSPACE/protos/*.proto
done

echo "generate code for grpc-web"
export JS_OUT=$WORKSPACE/dashboard/src/protocols
if [ -d $JS_OUT ]
then
    rm -r $JS_OUT
fi
mkdir -p $JS_OUT
$PROTOBUF_ROOT/bin/protoc -I $WORKSPACE/protos \
    -I $PROTOBUF_ROOT/include/google/protobuf \
    --js_out=import_style=commonjs:$JS_OUT \
    --grpc-web_out=import_style=commonjs,mode=grpcwebtext:$JS_OUT \
    $WORKSPACE/protos/*.proto

# ---------------------------------------------------------

if [ -d $WORKSPACE/models/src ]
then
    rm -r $WORKSPACE/models/src
fi
if [ -d $WORKSPACE/models/include ]
then
    rm -r $WORKSPACE/models/include
fi


# ---------------------------------------------------------
 
echo 'done.'
exit 0
