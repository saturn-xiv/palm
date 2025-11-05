#!/bin/bash

set -e

export WORKSPACE=$PWD

function generate_daisy() {
    cd $WORKSPACE/daisy/

    declare -a items=("crypto" "email" "rbac" "s3" "sms")
    for i in "${items[@]}"
    do
        echo "generate $i for daisy"
        local target=$i/v2
        mkdir -p $target
        protoc --go_out=$target --go_opt=paths=import --go-grpc_out=$target --go-grpc_opt=paths=import proto/$i.proto
    done    
}

generate_daisy

echo 'done.'
exit 0
