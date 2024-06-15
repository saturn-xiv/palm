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

if [ "$ID" != "ubuntu" ]; then
    echo "only ubuntu was supported"
    exit 1
fi

if [ "$#" -ne 1 ]; then
    echo "USAGE: $0 PROJECT"
    exit 1
fi

# https://launchpad.net/~rabbitmq/+archive/ubuntu/rabbitmq-erlang
# https://elixir-lang.org/install.html#precompiled-package
if [ ! -d $HOME/local/elixir ]; then
    apt update
    apt install -y software-properties-common
    add-apt-repository -y ppa:rabbitmq/rabbitmq-erlang
    apt install -y erlang git wget unzip locales locales-all
    echo "en_US.UTF-8 UTF-8" >/etc/locale.gen
    locale-gen
    update-locale LANG=en_US.UTF-8

    wget -q -P /tmp/ https://github.com/elixir-lang/elixir/releases/download/v1.17.0/elixir-otp-26.zip
    mkdir -p $HOME/local/elixir
    cd $HOME/local/elixir/
    unzip /tmp/elixir-otp-26.zip
fi

export LANG=en_US.UTF-8
export LC_ALL=en_US.UTF-8
export LANGUAGE=en_US.UTF-8
export ELIXIR_HOME=$HOME/local/elixir
export PATH=$ELIXIR_HOME/bin:$PATH

mix local.hex --force
mix local.rebar --force

build $1

echo 'done.'
exit 0
