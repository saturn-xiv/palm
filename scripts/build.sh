#!/bin/bash

set -e

source /etc/os-release

export WORKSPACE=$PWD
export VERSION="$(git describe --tags --always --dirty --first-parent)"
export TARGET=$WORKSPACE/tmp/palm-${VERSION}

# -----------------------------------------------------------------------------

function build_dashboard() {
    cd $WORKSPACE/$1/dashboard/
    if [ ! -d node_modules ]
    then
        npm install
    fi    
    npm run build
}

function build_java() {
    cd $WORKSPACE/$1/
    mvn clean
    mvn package -Dmaven.test.skip=true
}

function build_cpp() {
    local target=$TARGET/$2/usr/bin

    cd $WORKSPACE/$1/
    cmake --preset=$3
    cmake --build build/$3

    mkdir -p $target
    cp build/$3/$p $target/
}

# go tool dist list
function build_go() {
    cd $WORKSPACE/$1/

    local pkg="github.com/saturn-xiv/palm/$1/env"    
    local ldflags="-a -extldflags '-static' -s -w -X '$pkg.build_time=$(date -u -R)' -X '$pkg.git_version=$(git describe --tags --always --dirty --first-parent)'"
    local target=$TARGET/$2/usr/bin

    echo "build $1 on $3"
    mkdir -p $target
    CC=$3-linux-gnu-gcc CGO_ENABLED=0 GOOS=linux GOARCH=$2 go build -ldflags "$ldflags" -o $target/$1
}

function build_tulip_assets() {
    local target=$TARGET/$1/usr/share/palm/tulip
    mkdir -p $target

    cd $WORKSPACE/tulip/    
    if [ ! -d node_modules ]
    then
        npm install
    fi
    
    local -a items=(
        "@popperjs/core/dist/umd"
        "bootstrap/dist"
        "@tabler/core/dist"
        "@material/web"
        "bulma/css/bulma.min.css"
        "dayjs/dayjs.min.js"
        "dayjs/locale"
        "dayjs/plugin"
        "@fortawesome/fontawesome-free/css"
        "@fortawesome/fontawesome-free/js"
        "@fortawesome/fontawesome-free/sprites-full"
        "@fortawesome/fontawesome-free/svgs-full"
        "@fortawesome/fontawesome-free/webfonts"
    )
    for it in "${items[@]}"
    do
        local d=$(dirname $target/node_modules/$it)
        mkdir -p $d
        cp -r node_modules/$it $d/
    done

    cp -r db views assets $target/
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

declare -a architectures=("amd64" "arm64" "riscv64")

declare -a dashboard_projects=("tulip")
for p in "${dashboard_projects[@]}"
do
    build_dashboard $p

    for a in "${architectures[@]}"
    do
        mkdir -p $TARGET/$a/usr/share/palm/$p
        cp -r $WORKSPACE/$p/dashboard/dist $TARGET/$a/usr/share/palm/$p/dashboard
    done
done


declare -a go_projects=("daisy")
for p in "${go_projects[@]}"
do
    build_go $p amd64 x86_64
    build_go $p arm64 aarch64
    build_go $p riscv64 riscv64
done


declare -a java_projects=("camellia")
for p in "${java_projects[@]}"
do
    build_java $p

    for a in "${architectures[@]}"
    do
        mkdir -p $TARGET/$a/usr/share/palm/$p
        cp application-*.yml README.md target/$p-*.jar $TARGET/$a/usr/share/palm/$p/        
    done
done


declare -a cpp_projects=("tulip")
for p in "${cpp_projects[@]}"
do
    build_cpp $p amd64 x86_64
    build_cpp $p arm64 aarch64
    build_cpp $p riscv64 riscv64
done

for a in "${architectures[@]}"
do
    build_tulip_assets $a

    cd $WORKSPACE/    
    cp -r README.md LICENSE $TARGET/$a/usr/share/palm/    
    cp -r .debian $TARGET/$a/DEBIAN

    mkdir -p $TARGET/$a/var/lib/palm $TARGET/$a/etc/palm
    
    cd $TARGET/
    sed -i "7s/all/$a/g" $a/DEBIAN/control
    dpkg-deb --root-owner-group --build $a palm-${VERSION}_${a}.deb
done

echo "done($PACKAGE.tar.xz)."
exit 0
