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
    local target=$WORKSPACE/tulip/dashboard/src/grpc-web-client-gen
    if [ -d $target ]
    then
        rm -f $target/*.js $target/*.ts
    fi
    mkdir -p $target
    $PROTOBUF_HOME/bin/protoc -I $WORKSPACE/tulip/proto -I $PROTOBUF_HOME/include/google/protobuf \
        --js_out=import_style=commonjs,binary:$target \
        --grpc-web_out=import_style=typescript,mode=grpcweb:$target \
        $WORKSPACE/tulip/proto/*.proto

    # $PROTOBUF_HOME/bin/protoc -I $WORKSPACE/tulip/proto -I $PROTOBUF_HOME/include/google/protobuf \
    #     --js_out=import_style=commonjs,binary:$target \
    #     --grpc-web_out=import_style=commonjs+dts,mode=grpcweb:$target \
    #     $WORKSPACE/tulip/proto/*.proto

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

# https://www.twilio.com/docs/openapi/generating-a-rust-client-for-twilios-api#setup
# https://github.com/OpenAPITools/openapi-generator?tab=readme-ov-file#launcher-script
# https://openapi-generator.tech/docs/generators/rust/
function generate_twilio_rust_api() {
    local target=$WORKSPACE/twilio
    if [ ! -d $target ]
    then
        $HOME/.local/bin/openapi-generator-cli generate -g rust \
            -i https://raw.githubusercontent.com/twilio/twilio-oai/main/spec/json/twilio_api_v2010.json \
            -o $target \
            --additional-properties=useSingleRequestParameter=true,packageName=twilio,packageVersion=$(date +"%Y.%-m.%-d")
        find $target/src -type f -exec sed -i 's/models::models::/models::/g' {} +
        find $target/src -type f -exec sed -i '/\/\/\/$/d' {} +
        sed -i 's/models::serde_json::/serde_json::/g' $target/src/apis/api20100401_payment_api.rs
        cd $target/
        git apply $WORKSPACE/twilio.patch
        cargo clippy --fix --lib -p twilio    
    fi
}
# -----------------------------------------------------------------------------

generate_daisy
generate_tulip
generate_tulip_dashboard
generate_twilio_rust_api

echo "format cargo projects"
cd $WORKSPACE/
cargo fmt

echo 'done.'
exit 0
