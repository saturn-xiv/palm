#!/bin/bash

set -e

export GO_VERSION="1.22.4"
export OTP_VERSION="26"
export ELIXIR_VERSION="1.17.0"
export NODE_VERSION="20.14.0"

export DEBIAN_FRONTEND=noninteractive
export MIX_ENV=prod
export WORKSPACE=$PWD

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

source /etc/os-release

if [ "$ID" != "ubuntu" ]; then
    echo "only ubuntu was supported."
    exit 1
fi

if [ ! -f $HOME/downloads/dm-go-driver.zip ]; then
    echo "couldn't find dm driver for golang."
    exit 1
fi

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

    wget -q -P $HOME/downloads/ https://github.com/elixir-lang/elixir/releases/download/v${ELIXIR_VERSION}/elixir-otp-${OTP_VERSION}.zip
    mkdir -p $HOME/local/elixir
    unzip $HOME/downloads/elixir-otp-${OTP_VERSION}.zip -d $HOME/local/elixir
fi

export LANG=en_US.UTF-8
export LC_ALL=en_US.UTF-8
export LANGUAGE=en_US.UTF-8
export ELIXIR_HOME=$HOME/local/elixir
export PATH=$ELIXIR_HOME/bin:$PATH

mix local.hex --force
mix local.rebar --force

# -----------------------------------------------------------------------------

if [ ! -d $HOME/local/node-v${NODE_VERSION}-linux-x64 ]; then
    wget -P $HOME/downloads/ https://nodejs.org/dist/v${NODE_VERSION}/node-v${NODE_VERSION}-linux-x64.tar.xz
    wget -P $HOME/downloads/ https://nodejs.org/dist/v${NODE_VERSION}/node-v${NODE_VERSION}-linux-arm64.tar.xz
    tar -xf $HOME/downloads/node-v${NODE_VERSION}-linux-x64.tar.xz -C $HOME/local
fi

export NODE_HOME=$HOME/local/node-v${NODE_VERSION}-linux-x64
export PATH=$NODE_HOME/bin:$PATH

# -----------------------------------------------------------------------------

if [ ! -d $HOME/local/go ]; then
    wget -P $HOME/downloads/ https://go.dev/dl/go${GO_VERSION}.linux-amd64.tar.gz
    mkdir -p $HOME/local
    tar -C $HOME/local -xf $HOME/downloads/go${GO_VERSION}.linux-amd64.tar.gz
    unzip $HOME/downloads/dm-go-driver.zip -d $HOME/local/go/src
fi

export PATH=$HOME/local/go/bin:$PATH

# -----------------------------------------------------------------------------

export GIT_VERSION=$(git describe --tags --always --dirty --first-parent)
export PACKAGE_NAME="palm-$VERSION_CODENAME-$GIT_VERSION"
export TARGET_DIR=$WORKSPACE/tmp/$PACKAGE_NAME

if [ -f ${TARGET_DIR}.tar.xz ]; then
    echo "file ${TARGET_DIR}.tar.xz already exists."
    exit 1
fi

if [ -d $TARGET_DIR ]; then
    rm -r $TARGET_DIR
fi

# https://go.dev/src/internal/goarch/goarch.go
declare -a go_targets=(
    "amd64"
    "arm64"
    "riscv64"
    # "loong64"
)

declare -a go_projects=(
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

declare -a node_targets=(
    "x64"
    "arm64"
)

for t in "${node_targets[@]}"; do
    tar -xf $HOME/downloads/node-v${NODE_VERSION}-linux-${t}.tar.xz -C $TARGET_DIR
done

# -----------------------------------------------------------------------------

echo "build ${TARGET_DIR}.tar.xz ..."
tar -cf ${TARGET_DIR}.tar.xz -C $TARGET_DIR .
md5sum ${TARGET_DIR}.tar.xz >${TARGET_DIR}.md5

echo 'done.'
exit 0
