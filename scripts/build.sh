#!/bin/bash

set -e

source /etc/os-release

export WORKSPACE=$PWD
export VERSION="$(git describe --tags --always --dirty --first-parent)"
export PACKAGE="palm-$VERSION_CODENAME-$VERSION"
export TARGET=$WORKSPACE/tmp/$PACKAGE

# -----------------------------------------------------------------------------

function build_camellia() {
     cd $WORKSPACE/camellia/
    mvn clean
    mvn package -Dmaven.test.skip=true
    mkdir -p $TARGET/camellia
    cp application-pgsql.yml README.md target/camellia-*.jar $TARGET/camellia/
}

function build_hyacinth() {    
    cd $WORKSPACE/hyacinth/
    mvn clean
    mvn package -Dmaven.test.skip=true
    mkdir -p $TARGET/hyacinth/libs
    cp target/hyacinth-*.jar logback.xml README.md config-orig.toml $TARGET/hyacinth/

    cd $WORKSPACE/crocus/
    mvn clean
    mvn package -Dmaven.test.skip=true
    cp target/crocus-*.jar $TARGET/hyacinth/libs/
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

# https://www.debian.org/doc/debian-policy/ch-controlfields.html#debian-source-package-template-control-files-debian-control
function build_deb() {
    local package="${1}_${VERSION}_${2}.deb"
    echo "build $package"
    local target=$WORKSPACE/tmp/$1-$2-$VERSION/$1
    if [ -d $target ]
    then
        rm -rf $(dirname $target)
    fi
    
    mkdir -p $target/usr/bin
    cp $TARGET/bin/$3/$1 $target/usr/bin/

    cd $WORKSPACE/$1/

    mkdir -p $target/etc/nginx/sites-available
    cp etc/nginx.conf $target/etc/nginx/sites-available/loquat.conf
    mkdir -p $target/etc/systemd/system/
    cp etc/systemd/* $target/etc/systemd/system/

    mkdir -p $target/usr/share/$1    
    cp -r README.md $target/usr/share/$1/
    cp -r dashboard/dist $target/usr/share/$1/dashboard
    cp -r scripts/$1 $target/usr/share/$1/scripts
    cp -r scripts/DEBIAN $target/

    mkdir -p $target/etc/$1    

    mkdir -p $target/var/lib/$1
    chmod 400 $target/var/lib/$1

    cd $(dirname $target/)
    sed -i "7s/all/$2/g" $1/DEBIAN/control
    dpkg-deb --root-owner-group --build $1 $package
}

function build_cpp_x64() {
    echo "build cpp projects for $1"
    local build_root=$WORKSPACE/build/$VERSION_CODENAME-$1   

    # https://github.com/protocolbuffers/protobuf/issues/12185    
    cmake -DCMAKE_BUILD_TYPE=Release -G Ninja \
        -DABSL_ENABLE_INSTALL=ON \
        -DgRPC_BUILD_TESTS=OFF \
        -DCMAKE_TOOLCHAIN_FILE=$WORKSPACE/toolchains/$1.cmake -B $build_root -S $WORKSPACE
    cmake --build $build_root
    cd $build_root/

    mkdir -p $TARGET/bin/$1
    cp tulip/tulip $TARGET/bin/$1/
}

# -----------------------------------------------------------------------------
if [ "$ID" != "ubuntu" ]
then
    echo "unsupported system $ID"
    exit 1
fi

if [ -d $TARGET ]
then
    rm -r $TARGET
fi
mkdir $TARGET

build_cpp_x64 $(uname -m)

build_camellia
build_hyacinth

build_dashboard loquat

declare -a go_projects=("daisy" "loquat" "pansy")
for p in "${go_projects[@]}"
do
    build_go $p amd64 x86_64
    build_go $p arm64 aarch64
    build_go $p riscv64 riscv64
    # build_go $p loong64
done

build_deb loquat amd64 x86_64
build_deb loquat arm64 aarch64

cd $WORKSPACE/tmp/
if [ -f $PACKAGE.tar.xz ]
then
    rm $PACKAGE.tar.xz
fi
XZ_OPT='-9' tar -cJf $PACKAGE.tar.xz -C $(dirname $TARGET) $PACKAGE

echo "done($PACKAGE.tar.xz)."
exit 0
