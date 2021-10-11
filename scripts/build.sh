#!/bin/bash

set -e

export WORKSPACE=$PWD
export VERSION=$(git describe --tags --always --dirty --first-parent)
export GRPC_INSTALL_PREFIX=$HOME/.local
export CLANG_USE_STD="-stdlib=libstdc++"
export CMAKE_CLANG="-DCMAKE_C_COMPILER=clang-13 \
    -DCMAKE_CXX_COMPILER=clang++-13 \
    -DCMAKE_EXE_LINKER_FLAGS='-fuse-ld=lld-13'"
export CMAKE_OPTIONS="-DINSTALL_SHARED=OFF \
    -DLIBSERIAL_BUILD_DOCS=OFF \
    -DLIBSERIAL_ENABLE_TESTING=OFF \
    -DLIBSERIAL_PYTHON_ENABLE=OFF \
    -DLIBSERIAL_BUILD_EXAMPLES=OFF \
    -DINJA_BUILD_TESTS=OFF \
    -DFLATBUFFERS_BUILD_TESTS=OFF \
    -DBUILD_SHARED=OFF \
    -DWITH_LIBBSD=OFF \
    -DWITH_LIBSODIUM=OFF \
    -DWITH_TLS=OFF \
    -DWITH_PERF_TOOL=OFF \
    -DZMQ_BUILD_TESTS=OFF \
    -DBUILD_SHARED_LIBS=OFF \
    -DBUILD_SHARED_LIBS=OFF \
    -DBUILD_TESTING=OFF \
    -DBUILD_BENCHMARK=OFF \
    -DBUILD_TEST=OFF \
    -DBUILD_DOC=OFF \
    -DgRPC_SSL_PROVIDER=package \
    -DgRPC_ZLIB_PROVIDER=package \
    -DgRPC_PROTOBUF_PROVIDER=module \
    -DgRPC_PROTOBUF_PACKAGE_TYPE=module \
    -Dprotobuf_BUILD_TESTS=OFF \
    -DProtobuf_PROTOC_EXECUTABLE=$GRPC_INSTALL_PREFIX/bin/protoc \
    -DgRPC_ABSL_PROVIDER=module \
    -DgRPC_BUILD_TESTS=OFF \
    -DYAML_BUILD_SHARED_LIBS=OFF \
    -DDISABLE_TESTS=ON \
    -DBUILD_STATIC_LIBS=ON \
    -DSQLITECPP_INTERNAL_SQLITE=OFF \
    -DCPR_BUILD_TESTS=OFF \
    -DCPR_FORCE_USE_SYSTEM_CURL=ON"
export CMAKE_CROSS_OPTIONS="-DENABLE_ACTIVERECORD_COMPILER=OFF \
    -DENABLE_PAGECOMPILER=OFF \
    -DENABLE_PAGECOMPILER_FILE2PAGE=OFF \
    -DFLATBUFFERS_BUILD_FLATC=OFF \
    -DJWT_BUILD_EXAMPLES=OFF"

grpc_install() {
    local grpc_version="v1.41.0"
    local protoc_version="3.17.3.0"
    local grpc_src=$HOME/downloads/grpc
    local grpc_build=$HOME/build/grpc-amd64
    
    if [ -f $GRPC_INSTALL_PREFIX/bin/protoc-$protoc_version ]
    then
        return
    fi
    
    if [ -d $grpc_src ]
    then
        cd $grpc_src
        git checkout --recurse-submodules $grpc_version
    else
        git clone --recurse-submodules -b $grpc_version https://github.com/grpc/grpc.git $grpc_src
    fi
    
    if [ -d $grpc_build ]
    then
        rm -rv $grpc_build
    fi
    mkdir -pv $grpc_build
    cd $grpc_build
    cmake -DCMAKE_BUILD_TYPE=Release \
        -DgRPC_INSTALL=ON \
        -DgRPC_BUILD_TESTS=OFF \
        -DgRPC_SSL_PROVIDER=package \
        -DCMAKE_INSTALL_PREFIX=$GRPC_INSTALL_PREFIX $grpc_src
    make
    make install
        
}

dashboard_release() {
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

cross_clang_release() {
    echo "build ${1}@release..."
    mkdir -pv $WORKSPACE/build/${1}-clang-release
    cd $WORKSPACE/build/${1}-clang-release
    local target_flags="-target $2 -ccc-gcc-name $3"
    
    cmake $WORKSPACE -DCMAKE_BUILD_TYPE=Release \
        $CMAKE_CLANG $CMAKE_OPTIONS $CMAKE_CROSS_OPTIONS \
        -DCMAKE_C_COMPILER_TARGET=$2 -DCMAKE_CXX_COMPILER_TARGET=$2 \
        -DCMAKE_C_FLAGS="$target_flags" \
        -DCMAKE_CXX_FLAGS="$CLANG_USE_STD $target_flags" \
        -DCMAKE_TOOLCHAIN_FILE=$WORKSPACE/docker/$1.cmake
    make
}

amd64_clang_release() {
    echo 'build amd64@release...'
    mkdir -pv $WORKSPACE/build/amd64-clang-release
    cd $WORKSPACE/build/amd64-clang-release
    cmake $WORKSPACE -DCMAKE_BUILD_TYPE=Release \
        $CMAKE_CLANG $CMAKE_OPTIONS \
        -DCMAKE_CXX_FLAGS="$CLANG_USE_STD"
    make
}

amd64_clang_debug() {
    echo 'build amd64@debug...'
    mkdir -pv $WORKSPACE/build/amd64-clang-debug
    cd $WORKSPACE/build/amd64-clang-debug
    cmake $WORKSPACE -DCMAKE_BUILD_TYPE=Debug \
        $CMAKE_CLANG $CMAKE_OPTIONS \
        -DCMAKE_CXX_FLAGS="$CLANG_USE_STD"
    make
}

arch_clang_debug() {
    echo 'build arch@debug...'
    mkdir -pv $WORKSPACE/build/arch-clang-debug
    cd $WORKSPACE/build/arch-clang-debug
    cmake $WORKSPACE -DCMAKE_BUILD_TYPE=Debug \
        -DCMAKE_C_COMPILER=clang -DCMAKE_CXX_COMPILER=clang++ \
        -DCMAKE_EXE_LINKER_FLAGS='-fuse-ld=lld' \
        $CMAKE_OPTIONS \
        -DCMAKE_CXX_FLAGS="$CLANG_USE_STD"
    make
}

ubuntu_dependencies(){
    local boost_version="1.67"
    sudo apt install -y libpq-dev:$1 libmysqlclient-dev:$1 libsqlite3-dev:$1 \
        libcurl4-openssl-dev:$1 \
        libboost-system-dev${boost_version}:$1 libboost-locale-dev${boost_version}:$1 \
        libboost-program-options-dev${boost_version}:$1 \
        libboost-date-time-dev${boost_version}:$1 \
        libboost-chrono-dev${boost_version}:$1 \
        libboost-timer-dev${boost_version}:$1 libboost-random-dev${boost_version}:$1 \
        libboost-log-dev${boost_version}:$1 libboost-test-dev${boost_version}:$1
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
    local target=$WORKSPACE/tmp/palm-$1-$VERSION/target
    if [ -d $target ]
    then
        rm -rf $(dirname $target)
    fi
    cp -r $WORKSPACE/debian $target/

    mkdir -pv $target/usr/bin
    cp -av $WORKSPACE/build/$1-clang-release/apps/*  $target/usr/bin/

    mkdir -pv $TARGET/usr/share/palm
    cp -av $WORKSPACE/node_modules $target/usr/share/palm/
    cp -av $WORKSPACE/dashboard/dist $target/usr/share/palm/dashboard
    
    mkdir -pv $target/var/lib/palm
    mkdir -pv $target/lib/systemd/system/
    cp -av $WORKSPACE/scripts/palm.*.service $target/lib/systemd/system/

    mkdir -pv $target/etc/palm
    cp -r $WORKSPACE/LICENSE $WORKSPACE/README.md $WORKSPACE/package.json $target/etc/palm/
    echo "$VERSION $(date -R)" > $target/etc/palm/VERSION

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
    else
        CC=gcc
        CXX=g++
        export CC CXX
    fi
    
    cd $target
    dpkg-buildpackage -us -uc -b --host-arch $1
}

# -----------------------------------------------------------------------------

grpc_install

export OS_NAME=$(lsb_release -is)

if [[ $OS_NAME == "Ubuntu" ]]
then
    build_dashboard

    ubuntu_dependencies amd64
    amd64_clang_debug
    amd64_clang_release
    build_deb amd64

    ubuntu_dependencies armhf
    cross_clang_release armhf arm-linux-gnueabihf arm-linux-gnueabihf-gcc-10
    build_deb armhf

    ubuntu_dependencies arm64
    cross_clang_release arm64 aarch64-linux-gnu aarch64-linux-gnu-gcc-10
    build_deb arm64
elif [[ $OS_NAME == "Arch" ]]
then
    build_dashboard
    
    sudo pacman -S --needed postgresql-libs mariadb-libs boost
    arch_clang_debug
else
    echo "Unknowk os $OS_NAME"
    exit 1
fi


# dashboard_release

echo 'done.'
exit 0
