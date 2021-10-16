#!/bin/bash

set -e

# CLANG_HOME=$HOME/local/clang+llvm-13.0.0-x86_64-linux-gnu-ubuntu-20.04
# GCC_ARM_HOME=$HOME/local/gcc-arm-10.3-2021.07-x86_64-arm-none-linux-gnueabihf
# GCC_ARM64_HOME=$HOME/local/gcc-arm-10.3-2021.07-x86_64-aarch64-none-linux-gnu
# export CLANG_HOME GCC_ARM_HOME GCC_ARM64_HOME


export WORKSPACE=$PWD
export GIT_VERSION=$(git describe --tags --always --dirty --first-parent)
export VCPKG_HOME=$HOME/local/vcpkg
export CONAN_HOME=$WORKSPACE/docker/ubuntu/conan

export CMAKE_LIBSERIAL_OPTIONS="-DINSTALL_SHARED=OFF \
    -DLIBSERIAL_BUILD_DOCS=OFF \
    -DLIBSERIAL_ENABLE_TESTING=OFF \
    -DLIBSERIAL_PYTHON_ENABLE=OFF \
    -DLIBSERIAL_BUILD_EXAMPLES=OFF"

build_backend() {
    echo "build $1@$2..."
    mkdir -pv $WORKSPACE/build/$1-$2
    cd $WORKSPACE/build/$1-$2
    conan install --build=missing --profile:build=default \
        --profile:host=$CONAN_HOME/profiles/$1 $CONAN_HOME
    cmake $WORKSPACE -DCMAKE_BUILD_TYPE=$2 \
        $CMAKE_LIBSERIAL_OPTIONS \
        -DCMAKE_TOOLCHAIN_FILE=$CONAN_HOME/toolchains/$1.cmake
    make
}

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
    yarn build
}

build_deb(){
    local target=$WORKSPACE/tmp/palm-$1-$2-$GIT_VERSION/target
    if [ -d $target ]
    then
        rm -rf $(dirname $target)
    fi
    mkdir -pv $target
    cp -r $WORKSPACE/debian $target/

    mkdir -pv $target/usr/bin
    cd $WORKSPACE/build/$1-$2-Release/bin/
    cp -av fig mint pi $target/usr/bin/

    mkdir -pv $target/usr/share/palm
    cp -av $WORKSPACE/node_modules $target/usr/share/palm/
    cp -av $WORKSPACE/dashboard/dist $target/usr/share/palm/dashboard
    
    mkdir -pv $target/var/lib/palm
    mkdir -pv $target/lib/systemd/system/
    cp -av $WORKSPACE/scripts/palm.*.service $target/lib/systemd/system/

    mkdir -pv $target/etc/palm
    cp -r $WORKSPACE/LICENSE $WORKSPACE/README.md $WORKSPACE/package.json $target/etc/palm/
    echo "$GIT_VERSION $(date -R)" > $target/etc/palm/VERSION

    if [ "$1" = "armhf" ]
    then
        CC=arm-linux-gnueabihf-gcc
        CXX=arm-linux-gnueabihf-g++
        export CC CXX
    elif [ "$1" = "arm64" ]
    then
        CC=aarch64-linux-gnu-gcc
        CXX=aarch64-linux-gnu-g++
        export CC CXX
    elif [ "$1" = "amd64" ]
    then
        CC=gcc
        CXX=g++
        export CC CXX
    else
        echo "unknown arch $1"
        return 1
    fi
    
    cd $target
    dpkg-buildpackage -us -uc -b --host-arch $1
}

# -----------------------------------------------------------------------------

export OS_NAME=$(lsb_release -is)

if [[ $OS_NAME == "Ubuntu" ]]
then
    build_dashboard
    
    build_backend amd64-clang Release
    build_deb amd64 clang
    
    build_backend armhf-clang Release
    build_deb armhf clang

    build_backend arm64-clang Release
    build_deb arm64 clang
elif [[ $OS_NAME == "Arch" ]]
then
    build_dashboard
    build_backend arch-clang Debug
else
    echo "Unknowk os $OS_NAME"
    exit 1
fi


# dashboard_release

echo 'done.'
exit 0
