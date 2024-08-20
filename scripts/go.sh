#!/bin/bash

set -e

export WORKSPACE=$PWD

function build() {
    cd $WORKSPACE/$1

    local pkg="github.com/saturn-xiv/palm/$1/cmd"
    # ldflags="-extldflags=-static" -tags sqlite_omit_load_extension
    local ldflags="-s -w -X '$pkg.repo_url=$(git remote get-url origin)' -X '$pkg.author_name=$(git config --get user.name)' -X '$pkg.author_email=$(git config --get user.email)' -X '$pkg.build_time=$(date -u)' -X '$pkg.git_version=$(git describe --tags --always --dirty --first-parent)'"

    echo "build $1.$2"
    GOOS=linux GOARCH=$2 go build -ldflags "$ldflags" -o $WORKSPACE/$1/tmp/$1.$2
}

if [ "$#" -ne 1 ]; then
    echo "USAGE: $0 PROJECT"
    exit 1
fi

apt update
DEBIAN_FRONTEND=noninteractive apt install -y git wget

export GO_VERSION="1.23.0"
if [ ! -d $HOME/local/go ]; then
    wget -P /tmp/ https://go.dev/dl/go${GO_VERSION}.linux-amd64.tar.gz
    mkdir -p $HOME/local
    tar -C $HOME/local -xf /tmp/go${GO_VERSION}.linux-amd64.tar.gz
fi

export PATH=$HOME/local/go/bin:$PATH

# https://go.dev/src/internal/goarch/goarch.go
declare -a targets=(
    "amd64"
    "arm64"
    "riscv64"
    # "loong64"
)

cd $WORKSPACE/$1
# go mod tidy
mkdir -p $WORKSPACE/$1/tmp
for t in "${targets[@]}"; do
    build $1 $t
done

echo 'done'
exit 0
