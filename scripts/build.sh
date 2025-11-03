#!/bin/bash

set -e

export WORKSPACE=$PWD

function build_camellia() {
    cd $WORKSPACE/camellia/
    mvn clean
    mvn package -Dmaven.test.skip=true
}

function build_dashboard() {
    cd $WORKSPACE/$1/dashboard/
    if [ ! -d node_modules ]
    then
        npm install
    fi
    if [ -d dist ]
    then
        rm -r dist
    fi
    npm run build
}

function build_api() {
    sudo apt install -y libpq-dev libmysqlclient-dev libsqlite3-dev 
    cargo build --release -p $1
}

build_camellia
