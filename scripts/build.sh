#!/bin/bash

set -e

# CLANG_HOME=$HOME/local/clang+llvm-13.0.0-x86_64-linux-gnu-ubuntu-20.04
# GCC_ARM_HOME=$HOME/local/gcc-arm-10.3-2021.07-x86_64-arm-none-linux-gnueabihf
# GCC_ARM64_HOME=$HOME/local/gcc-arm-10.3-2021.07-x86_64-aarch64-none-linux-gnu
# export CLANG_HOME GCC_ARM_HOME GCC_ARM64_HOME


export WORKSPACE=$PWD
export GIT_VERSION=$(git describe --tags --always --dirty --first-parent)
export VCPKG_HOME=$HOME/local/vcpkg

# export GRPC_INSTALL_PREFIX=$HOME/.local
# export CLANG_USE_STD="-stdlib=libstdc++"
# export CMAKE_CLANG="-DCMAKE_C_COMPILER=clang-13 \
#     -DCMAKE_CXX_COMPILER=clang++-13 \
#     -DCMAKE_EXE_LINKER_FLAGS='-fuse-ld=lld-13'"

# grpc_install() {
#     local grpc_version="v1.41.0"
#     local protoc_version="3.17.3.0"
#     local grpc_src=$HOME/downloads/grpc
#     local grpc_build=$HOME/build/grpc-amd64
    
#     if [ -f $GRPC_INSTALL_PREFIX/bin/protoc-$protoc_version ]
#     then
#         return
#     fi
    
#     if [ -d $grpc_src ]
#     then
#         cd $grpc_src
#         git checkout --recurse-submodules $grpc_version
#     else
#         git clone --recurse-submodules -b $grpc_version https://github.com/grpc/grpc.git $grpc_src
#     fi
    
#     if [ -d $grpc_build ]
#     then
#         rm -rv $grpc_build
#     fi
#     mkdir -pv $grpc_build
#     cd $grpc_build
#     cmake -DCMAKE_BUILD_TYPE=Release \
#         -DgRPC_INSTALL=ON \
#         -DgRPC_BUILD_TESTS=OFF \
#         -DgRPC_SSL_PROVIDER=package \
#         -DCMAKE_INSTALL_PREFIX=$GRPC_INSTALL_PREFIX $grpc_src
#     make
#     make install
        
# }

# build_dashboard_release() {
#     cd $WORKSPACE
#     if [ ! -d node_modules ]
#     then
#         yarn install
#     fi
#     cd $WORKSPACE/dashboard
#     if [ ! -d node_modules ]
#     then
#         yarn install
#     fi
#     yarn build
# }

# cross_clang_release() {
#     echo "build ${1}@release..."
#     mkdir -pv $WORKSPACE/build/${1}-clang-release
#     cd $WORKSPACE/build/${1}-clang-release
#     local target_flags="-target $2 -ccc-gcc-name $3"
    
#     cmake $WORKSPACE -DCMAKE_BUILD_TYPE=Release \
#         $CMAKE_CLANG $CMAKE_OPTIONS $CMAKE_CROSS_OPTIONS \
#         -DCMAKE_C_COMPILER_TARGET=$2 -DCMAKE_CXX_COMPILER_TARGET=$2 \
#         -DCMAKE_C_FLAGS="$target_flags" \
#         -DCMAKE_CXX_FLAGS="$CLANG_USE_STD $target_flags" \
#         -DCMAKE_TOOLCHAIN_FILE=$WORKSPACE/docker/ubuntu/$1.cmake
#     make
# }

# amd64_clang_release() {
#     echo 'build amd64@release...'
#     mkdir -pv $WORKSPACE/build/amd64-clang-release
#     cd $WORKSPACE/build/amd64-clang-release
#     cmake $WORKSPACE -DCMAKE_BUILD_TYPE=Release \
#         $CMAKE_CLANG $CMAKE_OPTIONS \
#         -DCMAKE_CXX_FLAGS="$CLANG_USE_STD"
#     make
# }

# amd64_clang_debug() {
#     echo 'build amd64@debug...'
#     mkdir -pv $WORKSPACE/build/amd64-clang-debug
#     cd $WORKSPACE/build/amd64-clang-debug
#     cmake $WORKSPACE -DCMAKE_BUILD_TYPE=Debug \
#         $CMAKE_CLANG $CMAKE_OPTIONS \
#         -DCMAKE_CXX_FLAGS="$CLANG_USE_STD"
#     make
# }

build_arch_clang_debug() {
    echo 'build arch@debug...'
    mkdir -pv $WORKSPACE/build/arch-clang-debug
    cd $WORKSPACE/build/arch-clang-debug
    cmake $WORKSPACE -DCMAKE_BUILD_TYPE=Debug -DCMAKE_TOOLCHAIN_FILE=$VCPKG_HOME/scripts/toolchains/amd64.cmake
    make
}


build_dashboard_release(){
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
    mkdir -pv $target
    cp -r $WORKSPACE/debian $target/

    mkdir -pv $target/usr/bin
    cd $WORKSPACE/build/$1-clang-release/apps/
    cp -av fig mint pi $target/usr/bin/

    mkdir -pv $target/usr/share/palm
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

export OS_NAME=$(lsb_release -is)

if [[ $OS_NAME == "Ubuntu" ]]
then
    build_dashboard_release
    
    build_amd64_clang_release
    build_deb amd64
    
    build_armhf_clang_release
    build_deb armhf

    build_arm64_clang_release
    build_deb arm64
elif [[ $OS_NAME == "Arch" ]]
then
    build_dashboard_release
    build_arch_clang_debug
else
    echo "Unknowk os $OS_NAME"
    exit 1
fi


# dashboard_release

echo 'done.'
exit 0
