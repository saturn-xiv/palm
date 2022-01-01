#!/bin/bash

set -e

export GCC_VERSION=10
export WORKSPACE=$PWD
export GIT_VERSION=$(git describe --tags --always --dirty --first-parent)
export OS_NAME=$(lsb_release -is)

# -----------------------------------------------------------------------------
build_backend() {
    echo "build $1@$2..."
    mkdir -pv $WORKSPACE/build/$1-$2
    cd $WORKSPACE/build/$1-$2
    conan install --build=missing --profile:build=default \
        --profile:host=$WORKSPACE/docker/crosstool-ng/profiles/$1 $WORKSPACE/docker
    cmake $WORKSPACE -DCMAKE_BUILD_TYPE=$2 \
        -DCMAKE_TOOLCHAIN_FILE=$WORKSPACE/docker/crosstool-ng/toolchains/$1.cmake
    make -j
}
# -----------------------------------------------------------------------------
build_dashboard(){
    cd $WORKSPACE
    if [ ! -d node_modules ]
    then
        yarn install
    fi
    cd $WORKSPACE/dashboard
    if [ ! -d node_modules ]
    then
        yarn install
    fi
    # FIXME https://github.com/webpack/webpack/issues/14532
    NODE_OPTIONS=--openssl-legacy-provider yarn build
}
# -----------------------------------------------------------------------------

build_dashboard
    
if [[ $OS_NAME == "Ubuntu" ]]
then
    declare -a architectures=(
        "amd64"
        "arm64"
        "armhf"
    )
    for a in "${architectures[@]}"
    do
        build_backend $a Release
    done
elif [[ $OS_NAME == "Arch" ]]
    build_backend "arch" Debug
then
else
    echo "unknown os $OS_NAME"
    exit 1
fi

echo 'done.'
exit 0
