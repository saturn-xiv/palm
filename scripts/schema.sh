#!/bin/bash

set -e

export PROTOBUF_ROOT=$HOME/.local
export WORKSPACE=$PWD

function generate_gourd() {
    echo "generate gourd"
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

function generate_grpc_for_lang() {
    local target=$WORKSPACE/$2
    echo "generate grpc($1) => $2"
    if [ -d $target ]; then
        rm -r $target
    fi
    mkdir -p $target
    $PROTOBUF_ROOT/bin/protoc -I $WORKSPACE/petunia \
        -I $PROTOBUF_ROOT/include/google/protobuf \
        --${1}_out=$target --grpc_out=$target \
        --plugin=protoc-gen-grpc=$PROTOBUF_ROOT/bin/grpc_${1}_plugin \
        $WORKSPACE/petunia/*.proto
}

function generate_grpc_for_php() {
    echo "generate grpc for php"
    local target=$WORKSPACE/$1
    if [ -d $target ]; then
        rm -r $target
    fi
    mkdir -p $target
    $PROTOBUF_ROOT/bin/protoc -I $WORKSPACE/petunia \
        -I $PROTOBUF_ROOT/include/google/protobuf \
        --php_out=$target --grpc_out=generate_server:$target \
        --plugin=protoc-gen-grpc=$PROTOBUF_ROOT/bin/grpc_php_plugin \
        $WORKSPACE/petunia/*.proto
}

function generate_thrift_for_java() {
    echo "generate thrift-java for $1"
    local target=$2/com/github/saturn_xiv/palm/plugins/$1/v1
    if [ -d $target ]; then
        rm -r $target
    fi
    thrift -out $2 --gen java -r $PALM_PROTOCOLS/petunia/$1.thrift
}

function generate_lemon() {
    if [ -d $WORKSPACE/lemon/include ]; then
        rm -r $WORKSPACE/lemon/include
    fi
    if [ -d $WORKSPACE/lemon/src ]; then
        rm -r $WORKSPACE/lemon/src
    fi
    mkdir -p $WORKSPACE/lemon/src
    echo "generate lemon"

    $PROTOBUF_ROOT/bin/protoc -I $WORKSPACE/petunia \
        -I $PROTOBUF_ROOT/include/google/protobuf \
        --cpp_out=$WORKSPACE/lemon/src --grpc_out=$WORKSPACE/lemon/src \
        --plugin=protoc-gen-grpc=$PROTOBUF_ROOT/bin/grpc_cpp_plugin \
        $WORKSPACE/petunia/*.proto

    mkdir -p $WORKSPACE/lemon/include
    mv $WORKSPACE/lemon/src/*.h $WORKSPACE/lemon/include/
}

function generate_grpc_for_go() {
    echo "generate grpc $1 => $2"
    if [ -d $2 ]; then
        rm $2/*.pb.go
    else
        mkdir -p $2
    fi

    if [ ! -f $2/mod.go ]; then
        echo "package v2" >$2/mod.go
    fi

    $PROTOBUF_ROOT/bin/protoc -I $WORKSPACE/petunia \
        -I $PROTOBUF_ROOT/include/google/protobuf \
        --go_out=$2 --go_opt=paths=source_relative \
        --go-grpc_out=$2 --go-grpc_opt=paths=source_relative \
        $WORKSPACE/petunia/$1.proto
}

function generate_tutorials() {
    local java_target=$WORKSPACE/tutorials/java/src/main/java
    if [ -d $java_target/com/github/saturn_xiv/palm/plugins ]; then
        rm -r $java_target/com/github/saturn_xiv/palm/plugins
    fi
    $PROTOBUF_ROOT/bin/protoc -I $WORKSPACE/petunia \
        -I $PROTOBUF_ROOT/include/google/protobuf \
        --java_out=$java_target --grpc_out=$java_target \
        --plugin=protoc-gen-grpc=$PROTOBUF_ROOT/bin/grpc_java_plugin \
        $WORKSPACE/petunia/*.proto
}
# -----------------------------------------------------------------------------

generate_gourd
generate_lemon
generate_grpc_for_go balsam atropa/balsam/services/v2
generate_grpc_for_go daisy atropa/daisy/services/v2
generate_grpc_for_go s3 atropa/s3/services/v2
generate_grpc_for_go rbac atropa/rbac/services/v2
generate_grpc_for_go google atropa/google/services/v2
generate_grpc_for_go wechat atropa/wechat/services/v2
generate_grpc_for_php lemon/php

declare -a langs=(
    "csharp"
    "java"
    "python"
    "ruby"
)
for l in "${langs[@]}"; do
    generate_grpc_for_lang $l lemon/$l
done

generate_tutorials

echo 'done.'

exit 0
