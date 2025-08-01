#!/bin/bash

set -e

function build_app_image(){
    local target=tmp/AppDir
    if [ -d $target ]
    then
        rm -rf $target
    fi
    linuxdeploy-$1.AppImage --appdir $target -e build/$1/$2/$2 -d $2/assets/$2.desktop -i $2/assets/$2.png --output appimage
}

if [ "$#" -ne 1 ]; then
    echo "USAGE: $0 APP_NAME"
    exit 1
fi

build_app_image $(uname -m) $1
exit 0
