#!/bin/bash

set -e

export WORKSPACE=$PWD
export PROTOBUF_HOME=$HOME/.local

function generate_grpc_for_go() {
    echo "generate protocols($2) for $1"
    local target=$2/v2
    if [ -d $target ]
    then
        rm -f $target/*.pb.go
    else
        mkdir -p $target
    fi
    $PROTOBUF_HOME/bin/protoc --go_out=$target --go_opt=paths=import --go-grpc_out=$target --go-grpc_opt=paths=import proto/$2.proto
}

function generate_daisy() {
    cd $WORKSPACE/daisy/

    declare -a items=("portal" "crypto" "email" "rbac" "s3" "sms" "tex")
    for i in "${items[@]}"
    do
        generate_grpc_for_go daisy $i
    done    
}


function generate_loquat() {
    cd $WORKSPACE/loquat/

    generate_grpc_for_go loquat router    
}

function generate_gourd() {
    echo "generate protocols for gourd..."    
    local target=$WORKSPACE/gourd
    if [ -d $target ]
    then
        rm -r $target
    fi
    mkdir -p $target/include $target/src

    $PROTOBUF_HOME/bin/protoc \
        -I $WORKSPACE/daisy/proto -I $WORKSPACE/tulip/proto \
        -I $PROTOBUF_HOME/include/google/protobuf \
        --cpp_out=$target --grpc_out=$target \
        --plugin=protoc-gen-grpc=$PROTOBUF_HOME/bin/grpc_cpp_plugin \
        $WORKSPACE/daisy/proto/*.proto $WORKSPACE/tulip/proto/*.proto
    mv $target/*.h $target/include/
    mv $target/*.cc $target/src/
}

generate_daisy
generate_loquat
generate_gourd

echo 'done.'
exit 0
