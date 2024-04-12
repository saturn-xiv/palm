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

. /etc/os-release

if ! [ -x "$(command -v go)" ]; then
    if [ $ID == "ubuntu" ]; then
        # if [ ! -f /etc/apt/sources.list.d/longsleep-ubuntu-golang-backports-$VERSION_CODENAME.list ]; then
        #     apt update
        #     apt install -y software-properties-common
        #     add-apt-repository -y ppa:longsleep/golang-backports
        # fi
        # golang-go
        apt update
        apt install -y wget git
    elif [ $ID == "centos" ]; then
        if [ ! -f /etc/yum.repos.d/ius.repo ]; then
            yum install -y \
                https://repo.ius.io/ius-release-el7.rpm \
                https://dl.fedoraproject.org/pub/epel/epel-release-latest-7.noarch.rpm
        fi
        yum update -y
        yum install -y wget git236

    elif [ $ID == "arch" ]; then
        sudo pacman -S go
    else
        echo "Unsupported system: $ID"
        exit 1
    fi
fi

export GO_VERSION="1.22.2"
if [ ! -d /opt/go ]; then
    wget -P /tmp/ https://go.dev/dl/go${GO_VERSION}.linux-amd64.tar.gz
    tar -C /opt -xf /tmp/go${GO_VERSION}.linux-amd64.tar.gz
fi
export PATH=/opt/go/bin:$PATH
# if [ $ID == "ubuntu" ]; then
#     apt update
#     apt install -y wget git
# elif [ $ID == "centos" ]; then

# else
#     echo "Unsupported system: $ID"
#     exit 1
# fi

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
