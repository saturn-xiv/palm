#!/bin/bash

set -e

export MIX_ENV=prod
export PACKAGE_NAME=$1-$(date "+%4Y%m%d%H%M%S")
export WORKSPACE=$PWD

function build() {
    cd $WORKSPACE/$1

    if [ -d _build ]; then
        rm -r _build
    fi

    mix deps.get --only prod
    mix compile
    MIX_ENV=prod mix assets.deploy
    mix phx.gen.release
    MIX_ENV=prod mix release

    mkdir -p tmp
    echo "build tmp/$PACKAGE_NAME.tar.xz ..."
    tar -cf tmp/$PACKAGE_NAME.tar.xz -C _build/prod/rel/tuberose .
    md5sum tmp/$PACKAGE_NAME.tar.xz >tmp/$PACKAGE_NAME.md5
}

if [ "$#" -ne 1 ]; then
    echo "USAGE: $0 PROJECT"
    exit 1
fi

apt update
DEBIAN_FRONTEND=noninteractive apt install -y erlang elixir git

build $1

echo 'done.'
exit 0
