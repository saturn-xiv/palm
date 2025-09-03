#!/bin/bash

set -e

export PROTOBUF_HOME=$HOME/.local

export WORK_DIR=$PWD
export PROTOCOLS_HOME=$WORK_DIR/protocols

# -----------------------------------------------------------------------------

function generate_gourd_thrift() {
    local target=$WORK_DIR/gourd

    echo "generate thrift protocols(cpp) for gourd..."
    rm -f $target/include/*.h $target/src/*.cpp
    thrift -out $target --gen cpp:no_skeleton -r $PROTOCOLS_HOME/*.thrift
    mv $target/*.h $target/include/
    mv $target/*.cpp $target/src/
}

function generate_gourd_grpc() {
    echo "generate grpc protocols(cpp) for gourd..."
    local target=$WORK_DIR/gourd

    $PROTOBUF_HOME/bin/protoc -I $PROTOCOLS_HOME \
        -I $PROTOBUF_HOME/include/google/protobuf \
        --cpp_out=$target --grpc_out=$target \
        --plugin=protoc-gen-grpc=$PROTOBUF_HOME/bin/grpc_cpp_plugin \
        $PROTOCOLS_HOME/*.proto
    mv $target/*.h $target/include/
    mv $target/*.cc $target/src/
}


function generate_phlox_dashboard() {
    echo "generate grpc protocols(typescript) for phlox..."
    local target=$WORK_DIR/phlox/dashboard/src/protocols
    if [ -d $target ]
    then
        rm -r $target
    fi
    mkdir -p $target
    $PROTOBUF_HOME/bin/protoc -I $PROTOCOLS_HOME \
        -I $PROTOBUF_HOME/include/google/protobuf \
        --js_out=import_style=commonjs,binary:$target \
        --grpc-web_out=import_style=typescript,mode=grpcweb:$target \
        portal.proto monitoring.proto
}

function generate_crocus() {
    echo "generate grpc protocols(java) for crocus..."
    local target=$WORK_DIR/crocus/src/main/java/
    if [ -d $target/com/github/saturn_xiv/palm/plugins ]
    then
        rm -r $target/com/github/saturn_xiv/palm/plugins
    fi
    mkdir -p $target
    $PROTOBUF_HOME/bin/protoc -I $PROTOCOLS_HOME \
        -I $PROTOBUF_HOME/include/google/protobuf \
        --java_out=$target --grpc_out=$target \
        --plugin=protoc-gen-grpc=$PROTOBUF_HOME/bin/grpc_java_plugin \
        $PROTOCOLS_HOME/*.proto
}

function generate_jasmine() {
    echo "generate grpc protocols(go) for jasmine.$1..."
    local target=$WORK_DIR/jasmine/services/$1/v2
    if [ -d $target ]; then
        rm -f $target/*.pb.go
    else
        mkdir -p $target
    fi

    if [ ! -f $target/mod.go ]; then
        echo "package v2" >$target/mod.go
    fi

    $PROTOBUF_HOME/bin/protoc -I $PROTOCOLS_HOME \
        -I $PROTOBUF_HOME/include/google/protobuf \
        --go_out=$target --go_opt=paths=source_relative \
        --go-grpc_out=$target --go-grpc_opt=paths=source_relative \
        $PROTOCOLS_HOME/$1.proto
}

# -----------------------------------------------------------------------------

echo "clean gourd project"
if [ -d $WORK_DIR/gourd ]; then
    rm -r $WORK_DIR/gourd
fi
mkdir -p $WORK_DIR/gourd/include $WORK_DIR/gourd/src
generate_gourd_thrift
generate_gourd_grpc

# generate_phlox_dashboard

generate_crocus
generate_jasmine sms
generate_jasmine tex
generate_jasmine mail
generate_jasmine s3
generate_jasmine casbin
generate_jasmine portal
generate_jasmine wechat-pay
generate_jasmine wechat-subscription
generate_jasmine wechat-service
generate_jasmine wechat-miniprogram

echo 'done.'
exit 0
