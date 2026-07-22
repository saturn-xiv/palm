#!/bin/bash

set -e

export WORK_DIR=$PWD
export PROTOBUF_HOME=$HOME/local/protobuf

# https://grpc.io/docs/languages/rust/quickstart/#prerequisites
protoc --rust_opt=experimental-codegen=enabled,kernel=upb \
    -I $PROTOBUF_HOME/include/google/protobuf -I $WORK_DIR/protocols \
    --rust_out=$WORK_DIR/portal/src/protocols/ --rust-grpc_out=$WORK_DIR/portal/src/protocols/ \
    $WORK_DIR/protocols/casbin.proto $WORK_DIR/protocols/portal.proto

flatc -o portal/src/protocols --filename-suffix "" --rust protocols/email.fbs
flatc -o portal/src/protocols --filename-suffix "" --rust protocols/tex.fbs

export DATABASE_URL="postgres://postgres@127.0.0.1:5432/wisteria_dev?sslmode=disable"
diesel print-schema -o schema_migrations > $WORK_DIR/portal/src/schema.rs

cargo fmt

echo 'done.'
exit 0
