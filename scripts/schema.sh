#!/bin/bash

set -e

export PROTOBUF_HOME=$HOME/.local
export WORK_DIR=$PWD

function generate_loquat() {
    local target=$WORK_DIR/loquat

    echo "generate thrift protocols for loquat"
    rm -f $target/include/*.h $target/src/*.cpp
    thrift -out $target --gen cpp:no_skeleton -r $target/loquat.thrift
    mv $target/*.h $target/include/
    mv $target/*.cpp $target/src/
}

function generate_aloe() {
    local target=$WORK_DIR/aloe

    echo "generate db schema for loquat"
    DATABASE_URL=$target/db/aloe.sqlite3 diesel print-schema >$target/src/schema.rs
}

function generate_phlox() {
    local target=$WORK_DIR/phlox

    echo "generate protocols for phlox"
    thrift -out $target --gen rs -r $WORK_DIR/loquat/loquat.thrift
    mv $target/loquat.rs $target/src/loquat/v1.rs

    echo "generate db schema for phlox"
    DATABASE_URL="postgres://www:change-me@127.0.0.1:5432/phlox?sslmode=disable" diesel print-schema >$target/src/schema.rs
}

generate_loquat
generate_aloe
generate_phlox

cargo fmt

echo 'done.'
exit 0
