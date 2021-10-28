#!/bin/bash

set -e

export WORKSPACE=$PWD
export GIT_VERSION=$(git describe --tags --always --dirty --first-parent)
export VCPKG_HOME=$HOME/local/vcpkg
export CONAN_HOME=$WORKSPACE/docker/ubuntu/conan

export CMAKE_LIBSERIAL_OPTIONS="-DINSTALL_SHARED=OFF \
    -DLIBSERIAL_BUILD_DOCS=OFF \
    -DLIBSERIAL_ENABLE_TESTING=OFF \
    -DLIBSERIAL_PYTHON_ENABLE=OFF \
    -DLIBSERIAL_BUILD_EXAMPLES=OFF \
    -DCASBIN_BUILD_TEST=OFF \
    -DCASBIN_BUILD_BENCHMARK=OFF \
    -DCASBIN_BUILD_BINDINGS=OFF \
    -DCASBIN_BUILD_PYTHON_BINDINGS=OFF \
    -DCASBIN_INSTALL=OFF"

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
    # FIXME https://github.com/webpack/webpack/issues/14532
    NODE_OPTIONS=--openssl-legacy-provider yarn build
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
export OS_CODE=$(lsb_release -cs)

if [[ $OS_NAME == "Ubuntu" ]]
then
    build_dashboard
    
    if [[ $OS_CODE == "focal" ]]
    then
        build_backend amd64-clang Release
        build_deb amd64 clang
        
        build_backend armhf-clang Release
        # build_deb armhf clang

        build_backend arm64-clang Release
        # build_deb arm64 clang
    else
        build_backend amd64-clang Debug
    fi
elif [[ $OS_NAME == "Arch" ]]
then
    build_dashboard
    build_backend arch-clang Debug
else
    echo "Unknowk os $OS_NAME"
    exit 1
fi


echo 'done.'
exit 0
