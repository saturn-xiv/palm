#!/bin/bash

set -e

export WORKSPACE=$PWD
export PROTOBUF_HOME=$HOME/.local

 generate_grpc_for_go() {
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

    declare -a items=("rbac" "s3" "email" "sms" "tex" "cups")
    for i in "${items[@]}"
    do
        generate_grpc_for_go daisy $i
    done    
}

# function generate_crocus() {
#     echo "generate grpc protocols(java) for crocus"
#     local target=$WORKSPACE/crocus/src/main/java/
#     if [ -d $target/com/github/saturn_xiv/palm/plugins ]
#     then
#         rm -r $target/com/github/saturn_xiv/palm/plugins
#     fi
#     mkdir -p $target
#     $PROTOBUF_HOME/bin/protoc \
#         -I $WORKSPACE/daisy/proto -I $WORKSPACE/tulip/proto -I $WORKSPACE/camellia/src/main/proto \
#         -I $PROTOBUF_HOME/include/google/protobuf \
#         --java_out=$target --grpc_out=$target \
#         --plugin=protoc-gen-grpc=$PROTOBUF_HOME/bin/grpc_java_plugin \
#         $WORKSPACE/daisy/proto/*.proto $WORKSPACE/tulip/proto/*.proto $WORKSPACE/camellia/src/main/proto/*.proto
# }

# echo "generate marigold db-schema"
# diesel print-schema --database-url "postgres://www:change-me@127.0.0.1:5432/daisy_dev?sslmode=disable" > $WORKSPACE/marigold/src/schema.rs

# https://github.com/grpc/grpc-web?tab=readme-ov-file#typescript-support
function generate_tulip_dashboard() {
    echo "generate protocols for tulip-dashboard"
    local target=$WORKSPACE/tulip/dashboard/src/protocols
    if [ -d $target ]
    then
        rm -rf $target
    fi
    mkdir -p $target
    $PROTOBUF_HOME/bin/protoc -I $WORKSPACE/tulip/proto -I $PROTOBUF_HOME/include/google/protobuf \
        --js_out=import_style=commonjs,binary:$target \
        --grpc-web_out=import_style=typescript,mode=grpcweb:$target \
        $WORKSPACE/tulip/proto/*.proto
}

function generate_tulip() {
    echo "generate protocols for tulip"
    local target=$WORKSPACE/tulip/belladonna
    if [ -d $target ]
    then
        rm $target/include/*.h $target/src/*.cc
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

# -----------------------------------------------------------------------------

generate_daisy
generate_tulip
generate_tulip_dashboard

echo "format cargo projects"
cd $WORKSPACE/
cargo fmt

echo 'done.'
exit 0
