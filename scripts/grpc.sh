#!/bin/bash

set -e

export PROTOBUF_VERSION="34.0"
export RUST_PLUGIN_VERSION="0.9.0"
export PROTOBUF_HOME=$HOME/local/protobuf

if [ ! -f $HOME/downloads/protoc-${PROTOBUF_VERSION}-linux-x86_64.zip ]
then
    wget -P $HOME/downloads https://github.com/protocolbuffers/protobuf/releases/download/v${PROTOBUF_VERSION}/protoc-${PROTOBUF_VERSION}-linux-x86_64.zip
fi

if [ ! -f $HOME/downloads/protoc-gen-rust-grpc-${RUST_PLUGIN_VERSION}-linux-x86_64.zip ]
then
    wget -P $HOME/downloads https://github.com/grpc/grpc-rust/releases/download/protoc-gen-rust-grpc-v${RUST_PLUGIN_VERSION}/protoc-gen-rust-grpc-${RUST_PLUGIN_VERSION}-linux-x86_64.zip
fi

if [ -d $PROTOBUF_HOME ]
then
    echo "folder $PROTOBUF_HOME already exists."
    exit 1
fi

mkdir -p $PROTOBUF_HOME
unzip $HOME/downloads/protoc-${PROTOBUF_VERSION}-linux-x86_64.zip -d $PROTOBUF_HOME
unzip $HOME/downloads/protoc-gen-rust-grpc-${RUST_PLUGIN_VERSION}-linux-x86_64.zip -d $PROTOBUF_HOME

echo "done(protoc-${PROTOBUF_VERSION}, rust-($RUST_PLUGIN_VERSION))."
exit 0
