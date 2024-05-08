#!/bin/bash

set -e

export PROTOBUF_ROOT=$HOME/.local
export WORKSPACE=$PWD

function generate_thrift_for_cpp() {
    cd $WORKSPACE
    echo "generate $1 => $2"

    if [ -d $2/src ]; then
        rm -rv $2/src
    fi
    if [ -d $2/include ]; then
        rm -rv $2/include
    fi

    mkdir -p $2/src
    thrift -out $2/src --gen cpp:no_skeleton -r $1
    mkdir -p $2/include
    mv $2/src/*.h $2/include/
}

function generate_thrift_for_go() {
    cd $WORKSPACE
    echo "generate $1 => $2"

    if [ -d $2 ]; then
        for f in $2/*.go; do
            n=$(basename $f)
            if [[ "$n" != "mod.go" ]]; then
                rm -v $f
            fi
        done
    fi

    mkdir -p $2
    thrift -out $(dirname $2) --gen go:skip_remote,package=v1 -r $1
}

function generate_thrift_for_java() {
    cd $WORKSPACE
    echo "generate thrift $1 => $2"

    local target=$2/$3
    if [ -d $target ]; then
        rm -r $target
    fi
    thrift -out $2 --gen java:sorted_containers,jakarta_annotations,generated_annotations=undated -r $1
}

function generate_thrift_for_node() {
    cd $WORKSPACE
    echo "generate thrift $1 => $2"

    local target=$2
    if [ -d $target ]; then
        rm -r $target
    fi
    mkdir -p $2
    thrift -out $2 --gen js:node -r $1
}

function generate_thrift_for_rust() {
    cd $WORKSPACE
    echo "generate thrift $1 => $3"
    thrift -out tmp --gen rs -r $1
    mv tmp/$2.rs $3/protocols.rs
}

function generate_thrift_for_php() {
    cd $WORKSPACE
    echo "generate thrift $1 => $3"
    if [ -d $3/$2 ]; then
        rm -rv $3/$2
    fi
    thrift -out $3 --gen php:nsglobal=$2 -r $1
}

function generate_grpc_for_node() {
    echo "generate grpc $1 => $2"
    if [ -d $2 ]; then
        rm -rv $2
    fi
    mkdir -p $2
    grpc_tools_node_protoc -I $1 \
        -I $PROTOBUF_ROOT/include/google/protobuf \
        --js_out=import_style=commonjs,binary:$2 \
        --grpc_out=grpc_js:$2 $1/*.proto
}

function generate_diesel_postgresql_scheme() {
    echo "generate diesel schema for postgresql"

    DATABASE_URL=$1 diesel print-schema -o \
        locales settings \
        users user_contacts user_bans user_sessions logs \
        google_users \
        wechat_oauth2_users wechat_mini_program_users \
        attachments attachment_resources \
        leave_words shorter_links notifications \
        tags tag_resources \
        categories category_resources \
        vote_items vote_logs \
        footprints feedbacks favorites issues comments search_histories \
        menus \
        >camelia/src/schema.rs
    DATABASE_URL=$1 diesel print-schema -o schema_migrations >camelia/src/orm/postgresql/schema.rs
}

generate_thrift_for_cpp $WORKSPACE/loquat/loquat.thrift $WORKSPACE/loquat/gourd
generate_thrift_for_go $WORKSPACE/gourd/gourd.thrift $WORKSPACE/gourd/services/v1
generate_thrift_for_java $WORKSPACE/musa/wechat-pay.thrift $WORKSPACE/musa/src/main/java com/github/saturn_xiv/palm/plugins/musa/v1/wechat_pay
generate_thrift_for_node $WORKSPACE/morus/markdown.thrift $WORKSPACE/morus/src/protocols
generate_thrift_for_go $WORKSPACE/daisy/daisy.thrift $WORKSPACE/daisy/services/v1
generate_thrift_for_go $WORKSPACE/tuberose/tuberose.thrift $WORKSPACE/tuberose/services/v1
generate_thrift_for_go $WORKSPACE/jasmine/jasmine.thrift $WORKSPACE/jasmine/services/v1
generate_thrift_for_go $WORKSPACE/lily/lily.thrift $WORKSPACE/lily/services/v1
generate_thrift_for_go $WORKSPACE/jasmine/jasmine.thrift $WORKSPACE/lily/env/jasmine/v1
# hibiscus
generate_thrift_for_rust $WORKSPACE/jasmine/jasmine.thrift jasmine $WORKSPACE/hibiscus/src/jasmine
generate_thrift_for_rust $WORKSPACE/lily/lily.thrift lily $WORKSPACE/hibiscus/src/lily
generate_thrift_for_rust $WORKSPACE/gourd/gourd.thrift gourd $WORKSPACE/hibiscus/src/gourd
generate_thrift_for_rust $WORKSPACE/morus/markdown.thrift markdown $WORKSPACE/hibiscus/src/morus/markdown
generate_thrift_for_rust $WORKSPACE/musa/wechat-pay.thrift wechat-pay $WORKSPACE/hibiscus/src/musa/wechat_pay
generate_thrift_for_rust $WORKSPACE/tuberose/tuberose.thrift tuberose $WORKSPACE/hibiscus/src/tuberose
generate_thrift_for_rust $WORKSPACE/daisy/daisy.thrift daisy $WORKSPACE/hibiscus/src/daisy
generate_thrift_for_rust $WORKSPACE/loquat/loquat.thrift loquat $WORKSPACE/hibiscus/src/loquat
# tutorials
generate_thrift_for_php $WORKSPACE/loquat/loquat.thrift loquat $WORKSPACE/tutorials/php/lib
generate_thrift_for_php $WORKSPACE/lily/lily.thrift lily $WORKSPACE/tutorials/php/lib
generate_thrift_for_php $WORKSPACE/tuberose/tuberose.thrift tuberose $WORKSPACE/tutorials/php/lib
generate_thrift_for_php $WORKSPACE/daisy/daisy.thrift daisy $WORKSPACE/tutorials/php/lib
generate_thrift_for_php $WORKSPACE/morus/markdown.thrift 'morus\markdwown' $WORKSPACE/tutorials/php/lib
generate_thrift_for_php $WORKSPACE/musa/wechat-pay.thrift 'musa\wechat-pay' $WORKSPACE/tutorials/php/lib
generate_thrift_for_java $WORKSPACE/morus/markdown.thrift $WORKSPACE/tutorials/java/src/main/java com/github/saturn_xiv/palm/plugins/morus/v1/markdown

# postgresql
generate_diesel_postgresql_scheme "postgres://www:change-me@127.0.0.1:5432/palm?sslmode=disable"

cargo fmt
echo 'done.'
exit 0
