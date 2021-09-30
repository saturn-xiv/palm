#!/bin/bash

set -e

export WORKSPACE=$PWD
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
    -DBUILD_BENCHMARK=OFF"
export CMAKE_CROSS_OPTIONS="-DENABLE_ACTIVERECORD_COMPILER=OFF \
    -DENABLE_PAGECOMPILER=OFF \
    -DENABLE_PAGECOMPILER_FILE2PAGE=OFF \
    -DFLATBUFFERS_BUILD_FLATC=OFF"


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
    local target_flags="-target $1 -ccc-gcc-name $1-gcc"
    
    cmake $WORKSPACE -DCMAKE_BUILD_TYPE=Release \
        $CMAKE_CLANG $CMAKE_OPTIONS $CMAKE_CROSS_OPTIONS \
        -DCMAKE_C_COMPILER_TARGET=$1 -DCMAKE_CXX_COMPILER_TARGET=$1 \
        -DCMAKE_C_FLAGS="$target_flags" \
        -DCMAKE_CXX_FLAGS="$CLANG_USE_STD $target_flags" \
        -DCMAKE_TOOLCHAIN_FILE=$WORKSPACE/docker/armhf.cmake
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

export OS_NAME=$(lsb_release -is)
if [[ $OS_NAME == "Ubuntu" ]]
then
    sudo apt install -y libpq-dev libmysqlclient-dev
    amd64_clang_debug
    amd64_clang_release

    sudo apt install -y libpq-dev:armhf libmysqlclient-dev:armhf
    cross_clang_release arm-linux-gnueabihf
elif [[ $OS_NAME == "Arch" ]]
then
    sudo pacman -S --needed postgresql-libs mariadb-libs
    arch_clang_debug
else
    echo "Unknowk os $OS_NAME"
    exit 1
fi


# dashboard_release

echo 'done.'
exit 0
