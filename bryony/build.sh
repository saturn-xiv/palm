#!/bin/bash

set -e

export WORKSPACE=$PWD

# sudo apt-get install -y libcups2-dev

cmake cmake -DCMAKE_BUILD_TYPE=Release -G Ninja -B $WORKSPACE/build/bryony -S $WORKSPACE/bryony \
    -DBUILD_SHARED_LIBS=OFF \
    -DENABLE_SSL_SUPPORT=OFF \
    -DMAILIO_BUILD_TESTS=OFF -DMAILIO_BUILD_EXAMPLES=OFF -DMAILIO_BUILD_DOCUMENTATION=OFF

cmake --build $WORKSPACE/build/bryony
echo 'done.'
exit 0
