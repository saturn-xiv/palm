#!/bin/bash

set -e

export GCC_VERSION=10
export WORKSPACE=$PWD
export GIT_VERSION=$(git describe --tags --always --dirty --first-parent)
export CONAN_HOME=$WORKSPACE/docker/conan

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
    echo "build $1-$2@$3..."
    mkdir -pv $WORKSPACE/build/$1-$2-$3
    cd $WORKSPACE/build/$1-$2-$3
    conan install --build=missing --profile:build=default \
        --profile:host=$CONAN_HOME/profiles/$1/$2 $CONAN_HOME
    cmake $WORKSPACE -DCMAKE_BUILD_TYPE=$3 \
        $CMAKE_LIBSERIAL_OPTIONS \
        -DCMAKE_TOOLCHAIN_FILE=$CONAN_HOME/toolchains/$1/$2.cmake
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
    local target=$WORKSPACE/tmp/palm-$1-$2-Release-$GIT_VERSION/target
    if [ -d $target ]
    then
        rm -rf $(dirname $target)
    fi
    mkdir -pv $target
    cp -r $WORKSPACE/debian $target/

    mkdir -pv $target/usr/bin
    cd $WORKSPACE/build/$1-$2-Release/bin/
    cp -av fig mint $target/usr/bin/

    mkdir -pv $target/usr/share/palm
    cp -av $WORKSPACE/dashboard/dist $target/usr/share/palm/dashboard
    local -a packages=(
        "bootstrap/dist"
        "bulma/css"
        "marked/marked.min.js"
        "material-design-icons/iconfont"
        "moment/dist"
        "moment-timezone/builds/moment-timezone-with-data.min.js"
        "@popperjs/core/dist"
    )
    for i in "${packages[@]}"
    do
        local p=node_modules/$i
        local t=$(dirname "$target/usr/share/palm/$p")
        mkdir -p $t
        cp -av $WORKSPACE/$p $t/
    done
    
    mkdir -pv $target/var/lib/palm
    mkdir -pv $target/lib/systemd/system/
    
    mkdir -pv $target/etc/palm
    cp -r $WORKSPACE/LICENSE $WORKSPACE/README.md $WORKSPACE/package.json $target/etc/palm/
    echo "$GIT_VERSION $(date -R)" > $target/etc/palm/VERSION

    if [ "$2" = "armhf" ]
    then
        CC=arm-linux-gnueabihf-gcc-$GCC_VERSION
        CXX=arm-linux-gnueabihf-g++-$GCC_VERSION
        export CC CXX
    elif [ "$2" = "arm64" ]
    then
        CC=aarch64-linux-gnu-gcc-$GCC_VERSION
        CXX=aarch64-linux-gnu-g++-$GCC_VERSION
        export CC CXX
    elif [ "$2" = "amd64" ]
    then
        CC=gcc-$GCC_VERSION
        CXX=g++-$GCC_VERSION
        export CC CXX
    else
        echo "unknown arch $1"
        return 1
    fi
    
    cd $target
    dpkg-buildpackage -us -uc -b --host-arch $2
}

# -----------------------------------------------------------------------------

export OS_NAME=$(lsb_release -is)
export OS_CODE=$(lsb_release -cs)

if [[ $OS_NAME == "Ubuntu" ]]
then
    build_dashboard
    
    build_backend libstdc++ amd64 Debug
    build_backend libstdc++ amd64 Release
    build_deb libstdc++ amd64
    
    build_backend libstdc++ armhf Release
    # FIXME
    # build_deb libstdc++ armhf

    build_backend libstdc++ arm64 Release
    # FIXME
    # build_deb libstdc++ arm64
elif [[ $OS_NAME == "Arch" ]]
then
    build_dashboard
    build_backend libstdc++ arch Debug
    build_backend libc++ arch Debug
else
    echo "Unknowk os $OS_NAME"
    exit 1
fi


echo 'done.'
exit 0
