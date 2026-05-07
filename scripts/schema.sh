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


function generate_loquat() {
    echo "generate protocols for loquat"
    local target=$WORKSPACE/tulip/belladonna
}

function generate_thrift_for_cpp() {
    cd $WORKSPACE
    echo "generate $1 => $2"

    if [ -d $2/src ]; then
        rm -r $2/src
    fi
    if [ -d $2/include ]; then
        rm -r $2/include
    fi

    mkdir -p $2/src $2/include
    thrift -out $2/src --gen cpp:no_skeleton -r $1    
    mv $2/src/*.h $2/include/
}

# function generate_thrift_for_go() {
#     cd $WORKSPACE
#     echo "generate $1 => $2"

#     if [ -d $2 ]; then
#         for f in $2/*.go; do
#             n=$(basename $f)
#             if [[ "$n" != "mod.go" ]]; then
#                 rm -v $f
#             fi
#         done
#     fi

#     mkdir -p $2
#     thrift -out $(dirname $2) --gen go:skip_remote,package=v1 -r $1
# }

# function generate_thrift_for_java() {
#     cd $WORKSPACE
#     echo "generate thrift $1 => $2"

#     local target=$2/$3
#     if [ -d $target ]; then
#         rm -r $target
#     fi
#     thrift -out $2 --gen java:sorted_containers,jakarta_annotations,generated_annotations=undated -r $1
# }

# function generate_thrift_for_node() {
#     cd $WORKSPACE
#     echo "generate thrift $1 => $2"

#     local target=$2
#     if [ -d $target ]; then
#         rm -r $target
#     fi
#     mkdir -p $2
#     thrift -out $2 --gen js:node -r $1
# }

# function generate_thrift_for_rust() {
#     cd $WORKSPACE
#     echo "generate thrift $1/$2.thrift => $3"
#     mkdir -p $3
#     thrift -out tmp --gen rs -r $1/$2.thrift
#     mv tmp/$2.rs $3/protocols.rs
# }

# function generate_thrift_for_php() {
#     cd $WORKSPACE
#     echo "generate thrift $1 => $3"
#     if [ -d $3/$2 ]; then
#         rm -rv $3/$2
#     fi
#     thrift -out $3 --gen php:nsglobal=$2 -r $1
# }

# pip install grpcio-tools==1.76.0
function generate_dahlia() {
    cd $WORKSPACE/dahlia/src/
    echo "generate protocols for dahlia"
    local target=$WORKSPACE/dahlia/src/dahlia/protocols
    if [ -d $target ]
    then
        rm -r $target
    fi
    mkdir -p $target
    touch $target/__init__.py
    python -m grpc_tools.protoc \
        -Idahlia/protocols=$WORKSPACE/dahlia/proto -I $PROTOBUF_HOME/include/google/protobuf \
        --python_out=. --pyi_out=. --grpc_python_out=. \
        $WORKSPACE/dahlia/proto/*.proto
}
# -----------------------------------------------------------------------------

generate_daisy
generate_tulip
# generate_tulip_dashboard
generate_twilio_rust_api
generate_thrift_for_cpp $WORKSPACE/loquat/loquat.thrift $WORKSPACE/loquat/gourd
generate_dahlia

cd $WORKSPACE/
echo "format cargo projects"
cargo fmt
# echo "format cpp projects"
# clang-format -i loquat/include/loquat/*.hpp loquat/src/*.cpp

echo 'done.'
exit 0
