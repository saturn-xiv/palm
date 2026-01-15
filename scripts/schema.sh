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

    declare -a items=("portal" "crypto" "rbac" "s3" "email" "sms" "tex" "cups")
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
    echo "generate protocols(cpp) for gourd..."    
    local target=$WORKSPACE/gourd
    if [ -d $target ]
    then
        rm -r $target
    fi
    mkdir -p $target/include $target/src

    $PROTOBUF_HOME/bin/protoc \
        -I $WORKSPACE/daisy/proto -I $WORKSPACE/tulip/proto -I $WORKSPACE/camellia/src/main/proto \
        -I $PROTOBUF_HOME/include/google/protobuf \
        --cpp_out=$target --grpc_out=$target \
        --plugin=protoc-gen-grpc=$PROTOBUF_HOME/bin/grpc_cpp_plugin \
        $WORKSPACE/daisy/proto/*.proto $WORKSPACE/tulip/proto/*.proto $WORKSPACE/camellia/src/main/proto/*.proto
    mv $target/*.h $target/include/
    mv $target/*.cc $target/src/
}


function generate_crocus() {
    echo "generate grpc protocols(java) for crocus..."
    local target=$WORKSPACE/crocus/src/main/java/
    if [ -d $target/com/github/saturn_xiv/palm/plugins ]
    then
        rm -r $target/com/github/saturn_xiv/palm/plugins
    fi
    mkdir -p $target
    $PROTOBUF_HOME/bin/protoc \
        -I $WORKSPACE/daisy/proto -I $WORKSPACE/tulip/proto -I $WORKSPACE/camellia/src/main/proto \
        -I $PROTOBUF_HOME/include/google/protobuf \
        --java_out=$target --grpc_out=$target \
        --plugin=protoc-gen-grpc=$PROTOBUF_HOME/bin/grpc_java_plugin \
        $WORKSPACE/daisy/proto/*.proto $WORKSPACE/tulip/proto/*.proto $WORKSPACE/camellia/src/main/proto/*.proto
}

# https://github.com/grpc/grpc-web?tab=readme-ov-file#typescript-support
function generate_marigold() {
    local target=$WORKSPACE/marigold/dashboard/src/protocols
    if [ -d $target ]
    then
        rm -rf $target
    fi
    $PROTOBUF_HOME/bin/protoc \
        -I $WORKSPACE/daisy/proto -I $WORKSPACE/tulip/proto -I $WORKSPACE/camellia/src/main/proto \
        -I $PROTOBUF_HOME/include/google/protobuf \
        --js_out=import_style=commonjs,binary:$target \
        --grpc-web_out=import_style=typescript,mode=grpcweb:$target \
        $WORKSPACE/daisy/proto/*.proto $WORKSPACE/tulip/proto/*.proto $WORKSPACE/camellia/src/main/proto/*.proto
}

# -----------------------------------------------------------------------------

generate_daisy
generate_loquat
generate_gourd
generate_crocus

echo 'done.'
exit 0
