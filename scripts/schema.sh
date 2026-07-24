#!/bin/bash

set -e

export WORK_DIR=$PWD
export HYACINTH_OUTPUT_DIR=$WORK_DIR/hyacinth/src
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
# https://github.com/grpc/grpc-rust/tree/master/protoc-gen-rust-grpc
function generate_grpc() {
    echo "generate grpc protocols"

    rm -r $WORK_DIR/loquat/gourd/src $WORK_DIR/loquat/gourd/include
    mkdir $WORK_DIR/loquat/gourd/src $WORK_DIR/loquat/gourd/include
    $WORK_DIR/loquat/vcpkg/packages/protobuf_x64-linux-release/tools/protobuf/protoc \
        -I $WORK_DIR/loquat/proto -I $WORK_DIR/loquat/vcpkg/packages/protobuf_x64-linux-release/include/google/protobuf \
        --cpp_out=$WORK_DIR/loquat/gourd/src --grpc_out=$WORK_DIR/loquat/gourd/src \
        --plugin=protoc-gen-grpc=$WORK_DIR/loquat/vcpkg/packages/grpc_x64-linux/tools/grpc/grpc_cpp_plugin \
        $WORK_DIR/loquat/proto/*.proto
    mv $WORK_DIR/loquat/gourd/src/*.h $WORK_DIR/loquat/gourd/include/

    $PROTOBUF_HOME/bin/protoc --rust_opt=experimental-codegen=enabled,kernel=upb --rust-grpc_opt=client_only=true \
        --plugin=protoc-gen-rust-grpc=$PROTOBUF_HOME/bin/protoc-gen-rust-grpc \
        -I $PROTOBUF_HOME/include/ -I $WORK_DIR/marigold/src/main/proto \
        --rust_out=$HYACINTH_OUTPUT_DIR/wechatpay --rust-grpc_out=$HYACINTH_OUTPUT_DIR/wechatpay \
        $WORK_DIR/marigold/src/main/proto/wechatpay.proto
    
    $PROTOBUF_HOME/bin/protoc --rust_opt=experimental-codegen=enabled,kernel=upb --rust-grpc_opt=client_only=true \
        --plugin=protoc-gen-rust-grpc=$PROTOBUF_HOME/bin/protoc-gen-rust-grpc \
        -I $PROTOBUF_HOME/include/ -I $WORK_DIR/dahlia/proto/ \
        --rust_out=$HYACINTH_OUTPUT_DIR/rbac --rust-grpc_out=$HYACINTH_OUTPUT_DIR/rbac \
        $WORK_DIR/dahlia/proto/rbac.proto

    $PROTOBUF_HOME/bin/protoc --rust_opt=experimental-codegen=enabled,kernel=upb --rust-grpc_opt=client_only=true \
        --plugin=protoc-gen-rust-grpc=$PROTOBUF_HOME/bin/protoc-gen-rust-grpc \
        -I $PROTOBUF_HOME/include/ -I $WORK_DIR/loquat/proto/ \
        --rust_out=$HYACINTH_OUTPUT_DIR/loquat --rust-grpc_out=$HYACINTH_OUTPUT_DIR/loquat \
        $WORK_DIR/loquat/proto/loquat.proto
    
    # pip install 'grpcio-tools~=1.82'
    cd $WORK_DIR/dahlia/src/
    
    rm dahlia/protocols/*_pb2*

    PYTHON_GIL=0 python -m grpc_tools.protoc \
        -Idahlia/protocols=$WORK_DIR/dahlia/proto -I $PROTOBUF_HOME/include/google/protobuf \
        --python_out=. --pyi_out=. --grpc_python_out=. \
        $WORK_DIR/dahlia/proto/*.proto

    # $PROTOBUF_HOME/bin/protoc --rust_opt=experimental-codegen=enabled,kernel=upb --rust-grpc_opt=client_only=true \
    #     --plugin=protoc-gen-rust-grpc=$PROTOBUF_HOME/bin/protoc-gen-rust-grpc \
    #     -I $PROTOBUF_HOME/include/ -I $WORK_DIR/protocols/ \
    #     --rust_out=$HYACINTH_OUTPUT_DIR --rust-grpc_out=$HYACINTH_OUTPUT_DIR \
    #     $PROTOBUF_HOME/include/google/protobuf/empty.proto \
    #     $PROTOBUF_HOME/include/google/protobuf/timestamp.proto \
    #     $PROTOBUF_HOME/include/google/protobuf/duration.proto \
    #     $PROTOBUF_HOME/include/google/protobuf/any.proto
    

    # TODO 
    # $PROTOBUF_HOME/bin/protoc --rust_opt=experimental-codegen=enabled,kernel=upb --rust-grpc_opt=client_only=true \
    #     --rust-grpc_opt=extern_path=.google.protobuf=::crate::google::protobuf \
    #     --plugin=protoc-gen-rust-grpc=$PROTOBUF_HOME/bin/protoc-gen-rust-grpc \
    #     -I $PROTOBUF_HOME/include/ -I $WORK_DIR/protocols/ \
    #     --rust_out=$HYACINTH_OUTPUT_DIR/portal --rust-grpc_out=$HYACINTH_OUTPUT_DIR/portal \
    #     $WORK_DIR/protocols/portal.proto

}

function generate_flatbuffers() {
    echo "generate flatbuffers protocols"
    flatc -o $HYACINTH_OUTPUT_DIR --filename-suffix "" --rust $WORK_DIR/protocols/email.fbs
    flatc -o $HYACINTH_OUTPUT_DIR --filename-suffix "" --rust $WORK_DIR/protocols/tex.fbs
}

function generate_diesel() {
    echo "generate diesel schemas"
    local database_url="postgres://postgres@127.0.0.1:5432/wisteria_dev?sslmode=disable"
    diesel print-schema --database-url $database_url > $HYACINTH_OUTPUT_DIR/schema.rs
}

# function generate_thrift() {
#     echo "generate thrift protocols"
#     cd $WORK_DIR/loquat/gourd/
#     rm -rf include src

#     mkdir include src
#     thrift -out src --gen cpp:no_skeleton -r $WORK_DIR/protocols/loquat.thrift
#     mv src/*.h include/
    
#     thrift -out $HYACINTH_OUTPUT_DIR --gen rs -r $WORK_DIR/protocols/loquat.thrift
# }

generate_belladonna
generate_flatbuffers
generate_grpc
generate_diesel

sed -i -E "s/(version = \")[0-9]+\.[0-9]+\.[0-9]+/\1$(date +%Y.%-m.%-d)/g" $WORK_DIR/hyacinth/Cargo.toml

echo "format source codes"
cd $WORK_DIR/
cargo fmt
cd $WORK_DIR/loquat/
clang-format -i include/loquat/*.hpp src/*.cpp
cd $WORK_DIR/dahlia/
autopep8 --in-place --recursive src --exclude="src/dahlia/protocols/*,tmp/*"

echo 'done.'
exit 0
