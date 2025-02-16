#!/bin/bash

set -e

export PROTOBUF_ROOT=$HOME/.local
export WORKSPACE=$PWD

# function generate_grpc_for_java() {
#     local java_target=$WORKSPACE/tutorials/java/src/main/java
#     if [ -d $java_target/com/github/saturn_xiv/palm/plugins ]; then
#         rm -r $java_target/com/github/saturn_xiv/palm/plugins
#     fi
#     $PROTOBUF_ROOT/bin/protoc -I $PROTOCOLS_HOME \
#         -I $PROTOBUF_ROOT/include/google/protobuf \
#         --java_out=$java_target --grpc_out=$java_target \
#         --plugin=protoc-gen-grpc=$PROTOBUF_ROOT/bin/grpc_java_plugin \
#         $PROTOCOLS_HOME/*.proto
# }

# function generate_grpc_for_go() {
#     echo "generate grpc $1 => $2"
#     if [ -d $2 ]; then
#         rm $2/*.pb.go
#     else
#         mkdir -p $2
#     fi

#     if [ ! -f $2/mod.go ]; then
#         echo "package v2" >$2/mod.go
#     fi

#     $PROTOBUF_ROOT/bin/protoc -I $PROTOCOLS_HOME \
#         -I $PROTOBUF_ROOT/include/google/protobuf \
#         --go_out=$2 --go_opt=paths=source_relative \
#         --go-grpc_out=$2 --go-grpc_opt=paths=source_relative \
#         $PROTOCOLS_HOME/$1.proto
# }

# function generate_grpc_for_python() {
#     # pip install grpcio-tools
#     echo "generate grpc $2 => $1"
#     cd $WORKSPACE/$1
#     local target=palm/$2/v1
#     if [ -d $target ]; then
#         rm -r $target
#     fi
#     python -m grpc_tools.protoc -I$target=$PROTOCOLS_HOME \
#         -I $PROTOBUF_ROOT/include/google/protobuf \
#         --python_out=. --pyi_out=. --grpc_python_out=. $PROTOCOLS_HOME/$2.proto
# }

# function generate_grpc_for_php() {
#     echo "generate grpc for php"
#     local target=$WORKSPACE/$1
#     if [ -d $target ]; then
#         rm -r $target
#     fi
#     mkdir -p $target
#     $PROTOBUF_ROOT/bin/protoc -I $WORKSPACE/petunia \
#         -I $PROTOBUF_ROOT/include/google/protobuf \
#         --php_out=$target --grpc_out=generate_server:$target \
#         --plugin=protoc-gen-grpc=$PROTOBUF_ROOT/bin/grpc_php_plugin \
#         $1
# }

# https://github.com/grpc/grpc-node/tree/%40grpc/grpc-js%401.9.0/examples/helloworld/static_codegen
# function generate_grpc_for_js() {
#     # npm install -g grpc-tools
#     echo "generate gRPC for js $1 => $2"
#     if [ -d $2 ]; then
#         rm -r $2
#     fi
#     mkdir -p $2
#     grpc_tools_node_protoc -I $PROTOCOLS_HOME \
#         -I $PROTOBUF_ROOT/include/google/protobuf \
#         --js_out=import_style=commonjs,binary:$2 \
#         --grpc_out=grpc_js:$2 $1
# }

# generate_grpc_for_js morus/morus.proto morus/src/protocols

function generate_for_morus() {
    echo "generate for morus"
    local target=$WORKSPACE/morus/src/protocols
    if [ -d $target ]; then
        rm -r $target
    fi
    mkdir -p $target
    grpc_tools_node_protoc -I $WORKSPACE/morus \
        -I $PROTOBUF_ROOT/include/google/protobuf \
        --js_out=import_style=commonjs,binary:$target \
        --grpc_out=grpc_js:$target $WORKSPACE/morus/morus.proto
}

generate_for_morus

echo 'done.'
exit 0
