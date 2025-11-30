#!/bin/bash

set -e

export WORKSPACE=$PWD
export PACKAGE="palm-$(git describe --tags --always --dirty --first-parent)"
export TARGET=$WORKSPACE/tmp/$PACKAGE

function build_camellia() {
    cd $WORKSPACE/camellia/
    mvn clean
    mvn package -Dmaven.test.skip=true
    mkdir -p $TARGET/camellia
    cp application-*.yml target/camellia-*.jar $TARGET/camellia/
}

function build_dashboard() {
    cd $WORKSPACE/$1/dashboard/
    if [ ! -d node_modules ]
    then
        npm install
    fi
    if [ -d dist ]
    then
        rm -r dist
    fi
    npm run build
    mkdir -p $TARGET/$1
    cp -r dist $TARGET/$1/dashboard
}

function build_api() {
    sudo apt install -y libpq-dev libmysqlclient-dev libsqlite3-dev 
    cd $WORKSPACE/
    cargo build --release -p $1
}

# go tool dist list
function build_go() {
    cd $WORKSPACE/$1/

    local pkg="github.com/saturn-xiv/palm/$1/env"
    # ldflags="-extldflags=-static" -tags sqlite_omit_load_extension
    local ldflags="-s -w -X '$pkg.build_time=$(date -u -R)' -X '$pkg.git_version=$(git describe --tags --always --dirty --first-parent)'"

    echo "build $1.$2 on $3"
    mkdir -p $TARGET/bin/$3
    CC=$3-linux-gnu-gcc CGO_ENABLED=1 GOOS=linux GOARCH=$2 go build -ldflags "$ldflags" -o $TARGET/bin/$3/$1
}

function build_deb() {
    local target=$WORKSPACE/tmp/$1-$2-$(date +"%Y.%m.%d-p%H%M")/$1
    
    mkdir -p $target/usr/bin
    cp $TARGET/bin/$2/$1 $target/usr/bin/

    mkdir -p $target/usr/share/$1
    cd $WORKSPACE/$1/
    cp -r README.md $target/usr/share/$1/
    cp -r dashboard/dist $target/usr/share/$1/dashboard
    cp -r scripts/$1 $target/usr/share/$1/scripts
    cp -r scripts/DEBIAN $target/

    cd $target/
    dpkg-deb --build $1
}

if [ -d $TARGET ]
then
    rm -r $TARGET
fi
mkdir $TARGET

build_camellia
build_dashboard loquat

declare -a go_projects=("daisy" "loquat" "pansy")
for p in "${go_projects[@]}"
do
    build_go $p amd64 x86_64
    build_go $p arm64 aarch64
    build_go $p riscv64 riscv64
    # build_go $p loong64
done

build_deb loquat x86_64

cd $WORKSPACE/tmp/
if [ -f $PACKAGE.tar.xz ]
then
    rm $PACKAGE.tar.xz
fi
XZ_OPT='-9' tar -cJf $PACKAGE.tar.xz -C $(dirname $TARGET) $PACKAGE

echo "done($PACKAGE.tar.xz)."
exit 0
