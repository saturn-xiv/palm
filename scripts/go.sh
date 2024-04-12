#!/bin/bash

set -e

export WORKSPACE=$PWD

function build() {
    cd $WORKSPACE/$1

    local pkg="github.com/saturn-xiv/palm/$1/cmd"
    local ldflags="-extldflags=-static -s -w -X '$pkg.repo_url=$(git remote get-url origin)' -X '$pkg.author_name=$(git config --get user.name)' -X '$pkg.author_email=$(git config --get user.email)' -X '$pkg.build_time=$(date -u)' -X '$pkg.git_version=$(git describe --tags --always --dirty --first-parent)'"

    echo "build $1.$2"
    GOOS=linux GOARCH=$2 go build -ldflags "$ldflags" -tags sqlite_omit_load_extension -o $WORKSPACE/$1/tmp/$1.$2
}

if [ "$#" -ne 1 ]; then
    echo "USAGE: $0 PROJECT"
    exit 1
fi

. /etc/os-release

if ! [ -x "$(command -v go)" ]; then
    if [ $ID == "ubuntu" ]; then
        if [ ! -f /etc/apt/sources.list.d/longsleep-ubuntu-golang-backports-$VERSION_CODENAME.list ]; then
            apt update
            apt install -y software-properties-common
            add-apt-repository -y ppa:longsleep/golang-backports
        fi
        apt install -y git golang-go
    elif [ $ID == "arch" ]; then
        sudo pacman -S go
    else
        echo "Unsupported system: $ID"
        exit 1
    fi
fi

declare -a targets=(
    "amd64"
    "arm64"
    "riscv64"
)

cd $WORKSPACE/$1
go mod tidy
mkdir -p $WORKSPACE/$1/tmp
for t in "${targets[@]}"; do
    build $1 $t
done

echo 'done'
exit 0
