#!/bin/bash

set -e

apt update
apt install -y nodejs npm

if [ ! -d node_modules ]; then
    npm install
fi

npx webpack --mode=production

echo 'done.'
