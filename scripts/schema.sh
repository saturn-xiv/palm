#!/bin/bash

set -e

export PROTOBUF_ROOT=$HOME/.local
export WORKSPACE=$PWD
export PROTOCOLS_HOME=$WORKSPACE/petunia/protocols

# ---------------------------------------------------------

function generate_grpc_for_go() {
    echo "generate grpc $1 => $2"
    if [ -d $2 ]; then
        rm $2/*.pb.go
    else
        mkdir -p $2
    fi

    if [ ! -f $2/mod.go ]; then
        echo "package v2" >$2/mod.go
    fi

    $PROTOBUF_ROOT/bin/protoc -I $PROTOCOLS_HOME \
        -I $PROTOBUF_ROOT/include/google/protobuf \
        --go_out=$2 --go_opt=paths=source_relative \
        --go-grpc_out=$2 --go-grpc_opt=paths=source_relative \
        $PROTOCOLS_HOME/$1.proto
}

function generate_grpc_for_python() {
    # pip install grpcio-tools
    echo "generate grpc $2 => $1"
    cd $WORKSPACE/$1
    local target=palm/$2/v1
    if [ -d $target ]; then
        rm -r $target
    fi
    python -m grpc_tools.protoc -I$target=$PROTOCOLS_HOME \
        -I $PROTOBUF_ROOT/include/google/protobuf \
        --python_out=. --pyi_out=. --grpc_python_out=. $PROTOCOLS_HOME/$2.proto
}

# https://github.com/grpc/grpc-node/tree/%40grpc/grpc-js%401.9.0/examples/helloworld/static_codegen
function generate_grpc_for_js() {
    if [ -d $2 ]; then
        rm -r $2
    fi
    mkdir -p $2
    grpc_tools_node_protoc -I $PROTOCOLS_HOME \
        -I $PROTOBUF_ROOT/include/google/protobuf \
        --js_out=import_style=commonjs,binary:$2 \
        --grpc_out=grpc_js:$2 $PROTOCOLS_HOME/$1.proto
}

function generate_tutorials() {
    local java_target=$WORKSPACE/tutorials/java/src/main/java
    if [ -d $java_target/com/github/saturn_xiv/palm/plugins ]; then
        rm -r $java_target/com/github/saturn_xiv/palm/plugins
    fi
    $PROTOBUF_ROOT/bin/protoc -I $PROTOCOLS_HOME \
        -I $PROTOBUF_ROOT/include/google/protobuf \
        --java_out=$java_target --grpc_out=$java_target \
        --plugin=protoc-gen-grpc=$PROTOBUF_ROOT/bin/grpc_java_plugin \
        $PROTOCOLS_HOME/*.proto

    local php_target=$WORKSPACE/tutorials/php
    if [ -d $php_target/GPBMetadata ]; then
        rm -r $php_target/GPBMetadata
    fi
    if [ -d $php_target/Palm ]; then
        rm -r $php_target/Palm
    fi
    mkdir -p $php_target
    $PROTOBUF_ROOT/bin/protoc -I $PROTOCOLS_HOME \
        -I $PROTOBUF_ROOT/include/google/protobuf \
        --php_out=$php_target --grpc_out=generate_server:$php_target \
        --plugin=protoc-gen-grpc=$PROTOBUF_ROOT/bin/grpc_php_plugin \
        $PROTOCOLS_HOME/*.proto
}

function generate_diesel_schema() {

    cd $WORKSPACE/
    echo "generate database schema for daffodil"
    DATABASE_URL=$1 diesel print-schema -o locales settings \
        users logs sessions email_users google_oauth2_users wechat_oauth2_users wechat_mini_program_users \
        attachments attachment_resources \
        tags tag_resources \
        categories category_resources \
        menu_items leave_words >daffodil/src/schema.rs
    echo "generate database schema for carnation"
    DATABASE_URL=$1 diesel print-schema -o cms_pages >carnation/src/schema.rs
    echo "generate database schema for hibiscus"
    DATABASE_URL=$1 diesel print-schema -o forum forum_topics forum_posts >hibiscus/src/schema.rs
    echo "generate database schema for wisteria"
    DATABASE_URL=$1 diesel print-schema -o questionnaire_forms questionnaire_fields questionnaire_polls >wisteria/src/schema.rs
}

# ---------------------------------------------------------

# generate_grpc_for_go balsam atropa/balsam/services/v2
# generate_grpc_for_go daisy atropa/daisy/services/v2
# generate_grpc_for_go s3 atropa/s3/services/v2
# generate_grpc_for_go google atropa/google/services/v2
# generate_grpc_for_go wechat atropa/wechat/services/v2
# generate_grpc_for_go lily atropa/lily/services/v2
# generate_grpc_for_go morus atropa/morus/services/v2

# generate_grpc_for_js morus morus/src/protocols

# source $HOME/local/python/bin/activate
# generate_grpc_for_python bougainvillea rbac
# generate_grpc_for_python bougainvillea s3
# generate_grpc_for_python bougainvillea lily
# generate_grpc_for_python bougainvillea daisy

# generate_tutorials

# ---------------------------------------------------------

generate_diesel_schema "postgres://www:change-me@127.0.0.1:5432/palm?sslmode=disable"

cargo fmt

# ---------------------------------------------------------

echo 'done.'

exit 0

# function generate_gourd() {
#     echo "generate gourd"
#     if [ -d $WORKSPACE/gourd/include ]; then
#         rm -r $WORKSPACE/gourd/include
#     fi
#     if [ -d $WORKSPACE/gourd/src ]; then
#         rm -r $WORKSPACE/gourd/src
#     fi

#     mkdir -p $WORKSPACE/gourd/src
#     cd $WORKSPACE/
#     thrift -v -strict -out gourd/src --gen cpp:no_skeleton,include_prefix -r petunia/*.thrift
#     mkdir -p gourd/include/petunia
#     mv gourd/src/*.h gourd/include/petunia/
# }

# function generate_grpc_for_lang() {
#     local target=$WORKSPACE/$2
#     echo "generate grpc($1) => $2"
#     if [ -d $target ]; then
#         rm -r $target
#     fi
#     mkdir -p $target
#     $PROTOBUF_ROOT/bin/protoc -I $WORKSPACE/petunia \
#         -I $PROTOBUF_ROOT/include/google/protobuf \
#         --${1}_out=$target --grpc_out=$target \
#         --plugin=protoc-gen-grpc=$PROTOBUF_ROOT/bin/grpc_${1}_plugin \
#         $WORKSPACE/petunia/*.proto
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
#         $WORKSPACE/petunia/*.proto
# }

# function generate_thrift_for_java() {
#     echo "generate thrift-java for $1"
#     local target=$2/com/github/saturn_xiv/palm/plugins/$1/v1
#     if [ -d $target ]; then
#         rm -r $target
#     fi
#     thrift -out $2 --gen java -r $PALM_PROTOCOLS/petunia/$1.thrift
# }

# function generate_lemon() {
#     if [ -d $WORKSPACE/lemon/include ]; then
#         rm -r $WORKSPACE/lemon/include
#     fi
#     if [ -d $WORKSPACE/lemon/src ]; then
#         rm -r $WORKSPACE/lemon/src
#     fi
#     mkdir -p $WORKSPACE/lemon/src
#     echo "generate lemon"

#     $PROTOBUF_ROOT/bin/protoc -I $WORKSPACE/petunia \
#         -I $PROTOBUF_ROOT/include/google/protobuf \
#         --cpp_out=$WORKSPACE/lemon/src --grpc_out=$WORKSPACE/lemon/src \
#         --plugin=protoc-gen-grpc=$PROTOBUF_ROOT/bin/grpc_cpp_plugin \
#         $WORKSPACE/petunia/*.proto

#     mkdir -p $WORKSPACE/lemon/include
#     mv $WORKSPACE/lemon/src/*.h $WORKSPACE/lemon/include/
# }

# # https://github.com/grpc/grpc-web#code-generator-plugin
# function generate_grpc_web_for_typescript() {
#     echo "generate typescript sdk $1 => $2"
#     if [ -d $2 ]; then
#         rm -r $2
#     fi
#     mkdir -p $2

#     $PROTOBUF_ROOT/bin/protoc -I $WORKSPACE/petunia \
#         -I $PROTOBUF_ROOT/include/google/protobuf \
#         --js_out=import_style=commonjs,binary:$2 \
#         --grpc-web_out=import_style=typescript,mode=grpcweb:$2 \
#         $WORKSPACE/petunia/$1.proto
# }

# # -----------------------------------------------------------------------------

# generate_gourd
# generate_lemon

#
# generate_grpc_for_php lemon/php

# declare -a langs=(
#     "csharp"
#     "java"
#     "python"
#     "ruby"
# )
# for l in "${langs[@]}"; do
#     generate_grpc_for_lang $l lemon/$l
# done
