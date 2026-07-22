#!/bin/bash

set -e

export WORK_DIR=$PWD
export PROTOBUF_HOME=$HOME/local/protobuf


# https://www.twilio.com/docs/openapi/generating-a-rust-client-for-twilios-api#setup
# https://github.com/OpenAPITools/openapi-generator?tab=readme-ov-file#launcher-script
# https://openapi-generator.tech/docs/generators/rust/
function generate_belladonna() {
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
        # cargo clippy --fix --lib --allow-dirty -p belladonna
    fi
}

function generate_grpc() {
    # https://grpc.io/docs/languages/rust/quickstart/#prerequisites
    $PROTOBUF_HOME/bin/protoc --rust_opt=experimental-codegen=enabled,kernel=upb \
        --plugin=protoc-gen-rust-grpc=$PROTOBUF_HOME/bin/protoc-gen-rust-grpc \
        -I $PROTOBUF_HOME/include/ -I $WORK_DIR/protocols/ \
        --rust_out=$WORK_DIR/portal/src/protocols/ --rust-grpc_out=$WORK_DIR/portal/src/protocols/ \
        $PROTOBUF_HOME/include/google/protobuf/empty.proto $PROTOBUF_HOME/include/google/protobuf/timestamp.proto \
        $WORK_DIR/protocols/casbin.proto $WORK_DIR/protocols/portal.proto
}

function generate_flatbuffers() {
    flatc -o $WORK_DIR/portal/src/protocols --filename-suffix "" --rust $WORK_DIR/protocols/email.fbs
    flatc -o $WORK_DIR/portal/src/protocols --filename-suffix "" --rust $WORK_DIR/protocols/tex.fbs
}

function generate_diesel() {
    local database_url="postgres://postgres@127.0.0.1:5432/wisteria_dev?sslmode=disable"
    diesel print-schema --database-url $database_url -o schema_migrations > $WORK_DIR/portal/src/schema.rs
}

generate_belladonna
generate_flatbuffers
generate_grpc
generate_diesel


cargo fmt

echo 'done.'
exit 0
