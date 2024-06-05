#!/bin/bash

set -e

function install_deb() {
    apt install -y libsqlite3-dev:$1 libmysql++-dev:$1 libpqxx-dev:$1 \
        libssl-dev:$1 python3-dev:$1 libboost-all-dev:$1 libgrpc++-dev:$1
}

USAGE="USAGE: $0 amd64|arm64"
if [ "$#" -ne 1 ]; then
    echo $USAGE
    exit 1
fi

if [ $1 == "amd64" ]; then
    install_deb $1
    xmake f -p linux -a x86_64 -m release --toolchain=clang -P /workspace/saturn-xiv/palm-cpp/loquat
elif [ $1 == "arm64" ]; then
    install_deb $1
    # xmake f -p cross -m release --sdk=$HOME/local/arm-gnu-toolchain-13.2.Rel1-x86_64-aarch64-none-linux-gnu -P /workspace/saturn-xiv/palm-cpp/loquat
    xmake f -p cross --cross=aarch64-linux-gnu- -m release -P /workspace/saturn-xiv/palm-cpp/loquat
fi

echo 'done.'
exit 0
