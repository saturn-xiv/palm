#!/bin/bash

set -e

export WORKSPACE=$PWD

function generate_grpc_for_go() {
    echo "generate protocols($2) for $1"
    local target=$2/v2
    if [ -d $target ]
    then
        rm -f $target/*.pb.go
    else
        mkdir -p $target
    fi
    protoc --go_out=$target --go_opt=paths=import --go-grpc_out=$target --go-grpc_opt=paths=import proto/$2.proto
}

function generate_daisy() {
    cd $WORKSPACE/daisy/

    declare -a items=("crypto" "email" "rbac" "s3" "sms")
    for i in "${items[@]}"
    do
        generate_grpc_for_go daisy $i
    done    
}


function generate_loquat() {
    cd $WORKSPACE/loquat/

    generate_grpc_for_go loquat router    
}

generate_daisy
generate_loquat

echo 'done.'
exit 0
