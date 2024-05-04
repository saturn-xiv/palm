#!/bin/bash

set -e

export PROTOBUF_ROOT=$HOME/.local
export WORKSPACE=$PWD

function generate_thrift_for_cpp() {
    cd $WORKSPACE
    echo "generate $1 => $2"

    if [ -d $2/src ]; then
        rm -rv $2/src
    fi
    if [ -d $2/include ]; then
        rm -rv $2/include
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
        for f in $2/*.go; do
            n=$(basename $f)
            if [[ "$n" != "mod.go" ]]; then
                rm -v $f
            fi
        done
    fi

    mkdir -p $2
    thrift -out $(dirname $2) --gen go:skip_remote,package=v1 -r $1
}

function generate_thrift_for_java() {
    cd $WORKSPACE
    echo "generate thrift $1 => $2"

    echo "generate thrift-java for $1"
    local target=$2/$3
    if [ -d $target ]; then
        rm -r $target
    fi
    thrift -out $2 --gen java:sorted_containers,generated_annotations=undated -r $1
}

function generate_grpc_for_node() {
    echo "generate grpc $1 => $2"
    grpc_tools_node_protoc -I $1 \
        -I $PROTOBUF_ROOT/include/google/protobuf \
        --js_out=import_style=commonjs,binary:$2 \
        --grpc_out=grpc_js:$2 $1
}

generate_thrift_for_cpp $WORKSPACE/loquat/loquat.thrift $WORKSPACE/loquat/gourd
generate_thrift_for_go $WORKSPACE/gourd/gourd.thrift $WORKSPACE/gourd/services/v1
generate_thrift_for_java $WORKSPACE/musa/wechat-pay.thrift $WORKSPACE/musa/src/main/java com/github/saturn_xiv/palm/plugins/musa/v1/wechat_pay

if [ -d $WORKSPACE/morus/src/protocols ]; then
    rm -rv $WORKSPACE/morus/src/protocols
fi
mkdir -p $WORKSPACE/morus/src/protocols
generate_grpc_for_node $WORKSPACE/morus/markdown.proto $WORKSPACE/morus/src/protocols

echo 'done.'
exit 0
