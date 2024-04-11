#!/bin/bash

set -e

export PROTOBUF_ROOT=$HOME/.local
export WORKSPACE=$PWD
export PALM_PROTOCOLS=$WORKSPACE/palm/protocols

# -----------------------------------------------------------------------------

# function generate_grpc_by_lang() {
#     local target=$WORKSPACE/sdk/$1
#     echo "generate sdk for grpc-$1"
#     if [ -d $target ]; then
#         rm -r $target
#     fi
#     mkdir -p $target
#     $PROTOBUF_ROOT/bin/protoc -I $PALM_PROTOCOLS \
#         -I $PROTOBUF_ROOT/include/google/protobuf \
#         --${1}_out=$target --grpc_out=$target \
#         --plugin=protoc-gen-grpc=$PROTOBUF_ROOT/bin/grpc_${1}_plugin \
#         $PALM_PROTOCOLS/*.proto
# }

# function generate_grpc_for_cpp() {
#     echo "generate gRPC-cpp"
#     if [ -d $1/src ]; then
#         rm -r $1/src
#     fi
#     if [ -d $1/include ]; then
#         rm -r $1/include
#     fi

#     mkdir -p $1/src
#     $PROTOBUF_ROOT/bin/protoc -I $PALM_PROTOCOLS \
#         -I $PROTOBUF_ROOT/include/google/protobuf \
#         --cpp_out=$1/src --grpc_out=$1/src \
#         --plugin=protoc-gen-grpc=$PROTOBUF_ROOT/bin/grpc_cpp_plugin \
#         $PALM_PROTOCOLS/*.proto
#     mkdir -p $1/include
#     mv $1/src/*.h $1/include/
# }

# # https://github.com/grpc/grpc-web#code-generator-plugin
# function generate_grpc_for_typescript() {
#     echo "generate typescript sdk($1)"
#     if [ -d $1 ]; then
#         rm -r $1
#     fi
#     mkdir -p $1

#     $PROTOBUF_ROOT/bin/protoc -I $PALM_PROTOCOLS \
#         -I $PROTOBUF_ROOT/include/google/protobuf \
#         --js_out=import_style=commonjs,binary:$1 \
#         --grpc-web_out=import_style=typescript,mode=grpcweb:$1 \
#         $PALM_PROTOCOLS/*.proto
# }

# function generate_grpc_for_php() {
#     local target=$WORKSPACE/sdk/php
#     echo "generate sdk for grpc-php"
#     if [ -d $target ]; then
#         rm -r $target
#     fi
#     mkdir -p $target
#     $PROTOBUF_ROOT/bin/protoc -I $PALM_PROTOCOLS \
#         -I $PROTOBUF_ROOT/include/google/protobuf \
#         --php_out=$target --grpc_out=generate_server:$target \
#         --plugin=protoc-gen-grpc=$PROTOBUF_ROOT/bin/grpc_php_plugin \
#         $PALM_PROTOCOLS/*.proto
# }

# function generate_thrift_for_java() {
#     echo "generate thrift-java for $1"
#     local target=$2/com/github/saturn_xiv/palm/plugins/$1/v1
#     if [ -d $target ]; then
#         rm -r $target
#     fi
#     thrift -out $2 --gen java -r $PALM_PROTOCOLS/$1.thrift
# }

# function generate_thrift_for_cpp() {
#     cd $WORKSPACE

#     echo 'generate thrift-cpp for $1'
#     if [ -d $2/src ]; then
#         rm -r $2/src
#     fi
#     if [ -d $2/include ]; then
#         rm -r $2/include
#     fi

#     mkdir -p $2/src
#     thrift -out $2/src --gen cpp:no_skeleton -r $PALM_PROTOCOLS/$1.thrift
#     mkdir -p $2/include
#     mv $2/src/*.h $2/include/
# }

# function generate_gardenia() {
#     cd $WORKSPACE
#     local target=gardenia/src/main/java

#     echo "generate loquat protocol for gardenia"
#     local loquat_target=$target/com/github/saturn_xiv/palm/plugins/loquat/v1
#     if [ -d $loquat_target ]; then
#         rm -r $loquat_target
#     fi
#     thrift -out $target --gen java -r $PALM_PROTOCOLS/loquat.thrift

#     echo "generate gRPC for gardenia"
#     local gardenia_target=$target/com/github/saturn_xiv/palm/plugins/gardenia/v1
#     if [ -d $gardenia_target ]; then
#         rm -r $gardenia_target
#     fi
#     $PROTOBUF_ROOT/bin/protoc -I $PALM_PROTOCOLS \
#         -I $PROTOBUF_ROOT/include/google/protobuf \
#         --java_out=$target --grpc_out=$target \
#         --plugin=protoc-gen-grpc=$PROTOBUF_ROOT/bin/grpc_java_plugin \
#         $PALM_PROTOCOLS/gardenia.proto
# }

# -----------------------------------------------------------------------------

# DATABASE_URL=tmp/db diesel print-schema >ops/router/src/schema.rs

# declare -a languages=(
#     "python"
#     "ruby"
#     "csharp"
#     # https://repo1.maven.org/maven2/io/grpc/protoc-gen-grpc-java/
#     "java"
#     # "objective_c"
# )

# for l in "${languages[@]}"; do
#     generate_grpc_by_lang $l
# done

# generate_grpc_for_php
# generate_grpc_for_cpp $WORKSPACE/sdk/cpp
# generate_grpc_for_typescript $WORKSPACE/sdk/typescript

# generate_thrift_for_cpp loquat $WORKSPACE/loquat/gourd
#
# generate_gardenia
# generate_morus
# generate_lily

# -----------------------------------------------------------------------------
function generate_musa() {
    cd $WORKSPACE
    local target=musa/src/main/java

    echo "generate gRPC for musa"
    local musa_target=$target/com/github/saturn_xiv/palm/plugins/musa/v1
    if [ -d $musa_target ]; then
        rm -r $musa_target
    fi
    $PROTOBUF_ROOT/bin/protoc -I $PALM_PROTOCOLS \
        -I $PROTOBUF_ROOT/include/google/protobuf \
        --java_out=$target --grpc_out=$target \
        --plugin=protoc-gen-grpc=$PROTOBUF_ROOT/bin/grpc_java_plugin \
        $PALM_PROTOCOLS/musa.proto
}

function generate_loquat() {
    cd $WORKSPACE

    echo 'generate code for loquat'
    local cpp_target=loquat/gourd/src
    if [ -d $cpp_target ]; then
        rm -r $cpp_target
    fi
    mkdir -p $cpp_target
    thrift -out $cpp_target --gen cpp:no_skeleton -r $PALM_PROTOCOLS/loquat.thrift
}

# function generate_lily() {
#     echo "generate gRPC for lily"
#     local target=$WORKSPACE/lily/pumpkin
#     if [ -d $target ]; then
#         rm -r $target
#     fi
#     mkdir $target
#     touch $target/__init__.py

#     $PROTOBUF_ROOT/bin/protoc -I $WORKSPACE/lily \
#         -I $PROTOBUF_ROOT/include/google/protobuf \
#         --python_out=$target --grpc_out=$target \
#         --plugin=protoc-gen-grpc=$PROTOBUF_ROOT/bin/grpc_python_plugin \
#         $WORKSPACE/lily/lily.proto
#     sed -i 's/import lily_/from . import lily_/g' $target/lily_pb2_grpc.py

#     # echo "generate thrift for lily"
#     # local target=$WORKSPACE/tmp/lily-gen-python
#     # cd $WORKSPACE/tmp
#     # if [ -d $target ]; then
#     #     rm -r $target
#     # fi
#     # mkdir $target
#     # thrift -r -out $target --gen py $WORKSPACE/lily/lily.thrift

#     # local gourd_dir=$WORKSPACE/lily/gourd
#     # if [ -d $gourd_dir ]; then
#     #     rm -r $gourd_dir
#     # fi
#     # mv $target/lily/gourd $WORKSPACE/lily/gourd
# }

# function generate_go() {
#     local protocols_dir=$WORKSPACE/$1/protocols
#     local target=$WORKSPACE/$1/$2/v2

#     echo "generate $2 for $1"
#     mkdir -p $target
#     protoc -I $protocols_dir -I $PROTOBUF_ROOT/include/google/protobuf \
#         --go_out=$target --go_opt=paths=source_relative \
#         --go-grpc_out=$target --go-grpc_opt=paths=source_relative \
#         $protocols_dir/$2.proto
# }

function generate_morus() {
    echo "generate code for morus"
    local target=$WORKSPACE/morus/src/protocols

    if [ -d $target ]; then
        rm -r $target
    fi
    mkdir -p $target
    grpc_tools_node_protoc -I $PALM_PROTOCOLS \
        -I $PROTOBUF_ROOT/include/google/protobuf \
        --js_out=import_style=commonjs,binary:$target \
        --grpc_out=grpc_js:$target $PALM_PROTOCOLS/morus.proto
}

function generate_diesel_postgresql() {
    echo "generate diesel schema for postgresql"

    DATABASE_URL=$1 diesel print-schema \
        -o schema_migrations \
        casbin_rule locales settings \
        users user_contacts user_bans user_sessions logs \
        google_users \
        wechat_oauth2_users wechat_mini_program_users \
        attachments attachment_resources \
        roles roles_users roles_constraints permissions \
        leave_words shorter_links notifications twilio_sms_logs crawler_logs \
        tags tag_resources \
        categories category_resources \
        vote_items vote_logs \
        footprints feedbacks favorites issues comments search_histories \
        menus \
        >camelia/src/schema.rs
    DATABASE_URL=$1 diesel print-schema -o schema_migrations >camelia/src/orm/postgresql/schema.rs

    DATABASE_URL=$1 diesel print-schema -o cms_* >cms/src/schema.rs
    DATABASE_URL=$1 diesel print-schema -o forum_* >forum/src/schema.rs
    DATABASE_URL=$1 diesel print-schema -o daffodil_* >daffodil/src/schema.rs

}
# -----------------------------------------------------------------------------

generate_loquat
generate_musa
generate_morus

# generate_go lilac casbin
# generate_go lilac tink
# generate_go lilac minio

# generate_lily
# echo "generate lily requirements.txt"
# cd $WORKSPACE/lily
# pip freeze >requirements.txt

echo 'format rust code'
cd $WORKSPACE
generate_diesel_postgresql "postgres://www:change-me@127.0.0.1:5432/palm"
cargo fmt

echo 'done.'
exit 0
