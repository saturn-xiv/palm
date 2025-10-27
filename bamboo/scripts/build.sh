#!/bin/bash

set -e

export WORK_DIR=$PWD
export PACKAGE=bamboo-$(git describe --tags --always --dirty)
export BUILD_DIR=$PWD/tmp/$PACKAGE

function build_dashboard() {
    cd $WORK_DIR/dashboard/
   
    if [ ! -d node_modules ]
    then
        npm install 
    fi

    if [ -d dist ]
    then
        rm -r dist
    fi
    npm run build
    cp -r dist $BUILD_DIR/dashboard
}

function build_backend() {
    cd $WORK_DIR/
    
    mix deps.get --only prod
    MIX_ENV=prod mix phx.digest.clean --all

    MIX_ENV=prod mix compile
    MIX_ENV=prod mix assets.deploy

    MIX_ENV=prod mix phx.gen.release

    if [ -d _build/prod/rel/bamboo ]
    then
        rm -r _build/prod/rel/bamboo
    fi
    MIX_ENV=prod mix release

    cp -r _build/prod/rel/bamboo $BUILD_DIR/app
}

if [ -d $BUILD_DIR ]
then
    rm -r $BUILD_DIR
fi
mkdir -p $BUILD_DIR
build_dashboard
build_backend

XZ_OPT='-9' tar -cJf $PACKAGE.tar.xz -C $(dirname $BUILD_DIR) $PACKAGE
echo "done($PACKAGE.tar.xz)."
exit 0
