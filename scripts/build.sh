#!/bin/bash

set -e

export GO_VERSION="1.22.4"

export WORKSPACE=$PWD
export GIT_VERSION=$(git describe --tags --always --dirty --first-parent)
export PACKAGE_NAME="palm-$VERSION_CODENAME-$GIT_VERSION"
export TARGET_DIR=$WORKSPACE/tmp/$PACKAGE_NAME

# -----------------------------------------------------------------------------

function build_go() {
    cd $WORKSPACE/$1

    local pkg="github.com/saturn-xiv/palm/$1/cmd"
    # ldflags="-extldflags=-static" -tags sqlite_omit_load_extension
    local ldflags="-s -w -X '$pkg.repo_url=$(git remote get-url origin)' -X '$pkg.author_name=$(git config --get user.name)' -X '$pkg.author_email=$(git config --get user.email)' -X '$pkg.build_time=$(date -u)' -X '$pkg.git_version=$(git describe --tags --always --dirty --first-parent)'"

    echo "build $1.$2"
    mkdir -p $TARGET_DIR/bin
    GOOS=linux GOARCH=$2 go build -ldflags "$ldflags" -o $TARGET_DIR/bin/$1.$2
}

function build_dashboard() {
    cd $WORKSPACE/$1
    if [ ! -d node_modules ]; then
        npm install
    fi
    if [ -d dist ]; then
        rm -r dist
    fi
    npm run build
    mkdir -p $TARGET_DIR/$1
    cp -r dist locales $TARGET_DIR/$1/
}

function build_phoenix() {
    cd $WORKSPACE/$1

    if [ -d _build ]; then
        rm -r _build
    fi

    mix deps.get --only prod
    mix compile
    mix assets.deploy
    mix phx.gen.release
    mix release

    cp -r _build/prod/rel/$1 $TARGET_DIR/$1
}

# -----------------------------------------------------------------------------

# https://launchpad.net/~rabbitmq/+archive/ubuntu/rabbitmq-erlang
# https://elixir-lang.org/install.html#precompiled-package
if [ ! -d $HOME/local/elixir ]; then
    apt update
    apt install -y software-properties-common
    add-apt-repository -y ppa:rabbitmq/rabbitmq-erlang
    apt install -y build-essential erlang git wget unzip locales locales-all
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

# -----------------------------------------------------------------------------

if [ ! -d $HOME/local/go ]; then
    if [ ! -f /workspace/tmp/dm-go-driver.zip ]; then
        echo "couldn't find dm driver for golang."
        exit 1
    fi
    wget -P /tmp/ https://go.dev/dl/go${GO_VERSION}.linux-amd64.tar.gz
    mkdir -p $HOME/local
    tar -C $HOME/local -xf /tmp/go${GO_VERSION}.linux-amd64.tar.gz
    unzip /workspace/tmp/dm-go-driver.zip -d $HOME/local/go/src
fi

export PATH=$HOME/local/go/bin:$PATH

# -----------------------------------------------------------------------------

# https://go.dev/src/internal/goarch/goarch.go
declare -a go_targets=(
    "amd64"
    "arm64"
    "riscv64"
    # "loong64"
)

declare -a go_progjects=(
    "atropa"
    "sedge"
)

for t in "${go_targets[@]}"; do
    for p in "${go_projects[@]}"; do
        build_go $p $t
    done
done

build_dashboard jasmine
build_phoenix aloe
build_phoenix tuberose

# -----------------------------------------------------------------------------

echo "build ${TARGET_DIR}.tar.xz ..."
tar -cf ${TARGET_DIR}.tar.xz -C $TARGET_DIR .
md5sum ${TARGET_DIR}.tar.xz >${TARGET_DIR}.md5

echo 'done.'
exit 0
