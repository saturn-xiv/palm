#!/bin/bash

set -e

export WORK_DIR=$PWD
export OUTPUT_DIR=$WORK_DIR/hyacinth/src
export PROTOBUF_HOME=$HOME/local/protobuf

# https://www.twilio.com/docs/openapi/generating-a-rust-client-for-twilios-api#setup
# https://github.com/OpenAPITools/openapi-generator?tab=readme-ov-file#launcher-script
# https://openapi-generator.tech/docs/generators/rust/
function generate_belladonna() {
    echo "generate belladonna protocols"
    local target=$WORK_DIR/belladonna
    if [ ! -d $target ]
    then
        openapi-generator-cli generate -g rust \
            -i https://raw.githubusercontent.com/twilio/twilio-oai/main/spec/json/twilio_api_v2010.json \
            -o $target \
            --additional-properties=useSingleRequestParameter=true,packageName=belladonna,packageVersion=$(date +"%Y.%-m.%-d")
        
        find $target/src -type f -exec sed -i 's/models::models::/models::/g' {} +
        find $target/src -type f -exec sed -i '/\/\/\/$/d' {} +
        sed -i 's/models::serde_json::/serde_json::/g' $target/src/apis/api20100401_payment_api.rs
        
        cd $WORK_DIR/
        git apply patches/twilio.patch
        cargo clippy --fix --lib --allow-dirty --allow-staged -p belladonna
        sed -i '\|///$|d' belladonna/src/models/*.rs belladonna/src/apis/*.rs
    fi
}

# https://grpc.io/docs/languages/rust/quickstart/#prerequisites
function generate_grpc() {
    echo "generate grpc protocols"
        
    $PROTOBUF_HOME/bin/protoc --rust_opt=experimental-codegen=enabled,kernel=upb --rust-grpc_opt=client_only=true \
        --plugin=protoc-gen-rust-grpc=$PROTOBUF_HOME/bin/protoc-gen-rust-grpc \
        -I $PROTOBUF_HOME/include/ -I $WORK_DIR/protocols/ \
        --rust_out=$OUTPUT_DIR --rust-grpc_out=$OUTPUT_DIR \
        $PROTOBUF_HOME/include/google/protobuf/empty.proto \
        $PROTOBUF_HOME/include/google/protobuf/timestamp.proto \
        $PROTOBUF_HOME/include/google/protobuf/duration.proto \
        $PROTOBUF_HOME/include/google/protobuf/any.proto
    
    $PROTOBUF_HOME/bin/protoc --rust_opt=experimental-codegen=enabled,kernel=upb --rust-grpc_opt=client_only=true \
        --plugin=protoc-gen-rust-grpc=$PROTOBUF_HOME/bin/protoc-gen-rust-grpc \
        -I $PROTOBUF_HOME/include/ -I $WORK_DIR/protocols/ \
        --rust_out=$OUTPUT_DIR/casbin --rust-grpc_out=$OUTPUT_DIR/casbin \
        $WORK_DIR/protocols/casbin.proto

    $PROTOBUF_HOME/bin/protoc --rust_opt=experimental-codegen=enabled,kernel=upb --rust-grpc_opt=client_only=true \
        --plugin=protoc-gen-rust-grpc=$PROTOBUF_HOME/bin/protoc-gen-rust-grpc \
        -I $PROTOBUF_HOME/include/ -I $WORK_DIR/protocols/ \
        --rust_out=$OUTPUT_DIR/portal --rust-grpc_out=$OUTPUT_DIR/portal \
        $WORK_DIR/protocols/portal.proto

}

function generate_flatbuffers() {
    echo "generate flatbuffers protocols"
    flatc -o $OUTPUT_DIR --filename-suffix "" --rust $WORK_DIR/protocols/email.fbs
    flatc -o $OUTPUT_DIR --filename-suffix "" --rust $WORK_DIR/protocols/tex.fbs
}

function generate_diesel() {
    echo "generate diesel schemas"
    local database_url="postgres://postgres@127.0.0.1:5432/wisteria_dev?sslmode=disable"
    diesel print-schema --database-url $database_url > $OUTPUT_DIR/schema.rs
}

generate_belladonna
generate_flatbuffers
generate_grpc
generate_diesel

sed -i -E "s/(version = \")[0-9]+\.[0-9]+\.[0-9]+/\1$(date +%Y.%-m.%-d)/g" $WORK_DIR/hyacinth/Cargo.toml
cargo fmt

echo 'done.'
exit 0
