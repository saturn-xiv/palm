#!/bin/bash

set -e

mkdir build
cd build
apt install -y cmake g++-12 golang libunwind-dev libboost-all-dev
CC=gcc-10 CXX=g++-12 cmake -DCMAKE_BUILD_TYPE=Release \
    -DABSL_PROPAGATE_CXX_STD=ON -DTINK_USE_SYSTEM_OPENSSL=OFF \
    -DBUILD_COMPILER=OFF -DWITH_OPENSSL=OFF -DBUILD_JAVA=OFF -DBUILD_JAVASCRIPT=OFF -DBUILD_NODEJS=OFF -DBUILD_PYTHON=OFF \
    ..
make loquat

echo 'done.'
exit 0
