#/bin/bash

set -e

. /etc/os-release

if [ $ID == "ubuntu" && $VERSION_CODENAME == "jammy"]; then
    apt update
    apt install -y build-essential cmake g++-12 golang libunwind-dev libboost-all-dev
else
    echo "Unsupported system: $ID"
    exit 1
fi

export SOURCE_ROOT=$PWD
export BUILD_ROOT=$PWD/build/Release

mkdir -p $BUILD_ROOT

CC=gcc-12 CXX=g++-12 cmake -DCMAKE_BUILD_TYPE=Release \
    -DABSL_PROPAGATE_CXX_STD=ON -DTINK_USE_SYSTEM_OPENSSL=OFF \
    -DBUILD_COMPILER=OFF -DWITH_OPENSSL=OFF -DBUILD_JAVA=OFF -DBUILD_JAVASCRIPT=OFF -DBUILD_NODEJS=OFF -DBUILD_PYTHON=OFF \
    -B $BUILD_ROOT -S $SOURCE_ROOT

make -C $BUILD_ROOT loquat

echo 'done.'
exit 0
