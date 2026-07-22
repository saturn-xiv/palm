#!/bin/bash

set -e

export WORK_DIR=$PWD
export PROTOBUF_HOME=$HOME/local/protobuf

# https://grpc.io/docs/languages/rust/quickstart/#prerequisites
$PROTOBUF_HOME/bin/protoc --rust_opt=experimental-codegen=enabled,kernel=upb \
    --plugin=protoc-gen-rust-grpc=$PROTOBUF_HOME/bin/protoc-gen-rust-grpc \
    -I $PROTOBUF_HOME/include/ -I $WORK_DIR/protocols/ \
    --rust_out=$WORK_DIR/portal/src/protocols/ --rust-grpc_out=$WORK_DIR/portal/src/protocols/ \
    $PROTOBUF_HOME/include/google/protobuf/empty.proto $PROTOBUF_HOME/include/google/protobuf/timestamp.proto \
    $WORK_DIR/protocols/casbin.proto $WORK_DIR/protocols/portal.proto

flatc -o $WORK_DIR/portal/src/protocols --filename-suffix "" --rust $WORK_DIR/protocols/email.fbs
flatc -o $WORK_DIR/portal/src/protocols --filename-suffix "" --rust $WORK_DIR/protocols/tex.fbs

# https://www.twilio.com/docs/openapi/generating-a-rust-client-for-twilios-api
if [ ! -d $WORK_DIR/belladonna ]
then
    openapi-generator-cli generate -g rust \
    -i https://raw.githubusercontent.com/twilio/twilio-oai/main/spec/json/twilio_api_v2010.json \
    -o $WORK_DIR/belladonna \
    --additional-properties=useSingleRequestParameter=true
fi

export DATABASE_URL="postgres://postgres@127.0.0.1:5432/wisteria_dev?sslmode=disable"
diesel print-schema -o schema_migrations > $WORK_DIR/portal/src/schema.rs

cargo fmt

echo 'done.'
exit 0
