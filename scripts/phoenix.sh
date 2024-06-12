#!/bin/bash

set -e

export DEBIAN_FRONTEND=noninteractive
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
    mix assets.deploy
    mix phx.gen.release
    mix release

    mkdir -p tmp
    echo "build tmp/$PACKAGE_NAME.tar.xz ..."
    tar -cf tmp/$PACKAGE_NAME.tar.xz -C _build/prod/rel/$1 .
    md5sum tmp/$PACKAGE_NAME.tar.xz >tmp/$PACKAGE_NAME.md5
}

source /etc/os-release

if [ "$ID" -ne "ubuntu" ]; then
    echo "only ubuntu was supported"
    exit 1
fi

if [ "$#" -ne 1 ]; then
    echo "USAGE: $0 PROJECT"
    exit 1
fi

# https://elixir-lang.org/install.html#distributions
if [ ! -f /etc/apt/sources.list.d/rabbitmq-ubuntu-rabbitmq-erlang-${VERSION_CODENAME}.sources ]; then
    apt update
    apt install -y software-properties-common
    add-apt-repository -y ppa:rabbitmq/rabbitmq-erlang
    apt update
    apt install -y elixir erlang-dev erlang-xmerl git
fi

build $1

echo 'done.'
exit 0
