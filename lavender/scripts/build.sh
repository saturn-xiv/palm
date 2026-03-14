#!/bin/bash

set -e

xmake f -p cross -a arm64 --cross=aarch64-linux-gnu- -m release
xmake

xmake f -p cross -a x86_64 --cross=x86_64-linux-gnu- -m release
xmake

# xmake f -p cross -a riscv64 --cross=riscv64-linux-gnu- -m release
# xmake

echo 'done.'
