#!/bin/bash

set -e

export WORK_DIR=$PWD
export BUILD_DIR=$PWD/tmp/bamboo-$(git describe --tags --always --dirty)

function build_dashboard() {
    cd $WORK_DIR/dashboard/
   
    if [ ! -d node_modules ]
    then
        npm install 
    fi
    npm run build
    cp -r dist $BUILD_DIR/dashboard
}

function build_backend() {
    cd $WORK_DIR/
    
    mix deps.get --only prod
    mix phx.digest.clean --all

    MIX_ENV=prod mix compile
    MIX_ENV=prod mix assets.deploy

    mix phx.gen.release
    MIX_ENV=prod mix release

    cp -r _build/prod/rel/bamboo $BUILD_DIR/
}

if [ -d $BUILD_DIR ]
then
    rm -r $BUILD_DIR
fi
mkdir -p $BUILD_DIR
build_dashboard

echo "done($BUILD_DIR)."
exit 0
