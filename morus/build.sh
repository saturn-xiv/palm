#!/bin/bash

set -e

apt update
# DEBIAN_FRONTEND=noninteractive apt install -y nodejs npm \
#     pkg-config build-essential libpixman-1-dev libcairo2-dev librust-pangocairo-dev

export NODE_VERSION="20.12.2"

apt install -y wget xz-utils
if [ ! -d /opt/node-v${NODE_VERSION}-linux-x64 ]; then
    wget -q -P $HOME/ https://nodejs.org/dist/v${NODE_VERSION}/node-v${NODE_VERSION}-linux-x64.tar.xz
    tar xf $HOME/node-v${NODE_VERSION}-linux-x64.tar.xz -C /opt
fi

export NODE_HOME=/opt/node-v${NODE_VERSION}-linux-x64
export PATH=$NODE_HOME/bin:$PATH

if [ ! -d node_modules ]; then
    npm install
fi

npx webpack --mode=production

echo 'done.'
