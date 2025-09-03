#!/bin/bash

set -e


set -e

export SOURCE_ROOT=$HOME/downloads/thrift

function build_thrift() {    
    if [ -f /usr/local/bin/thrift ]; then
        /usr/local/bin/thrift --version
        echo 'already exists!'
        exit 0
    fi
    if [ -d $SOURCE_ROOT ]; then
        cd $SOURCE_ROOT
        git checkout master
        git pull
        git checkout $1       
    else
        git clone -b $1 https://github.com/apache/thrift.git $SOURCE_ROOT
    fi

    cd $SOURCE_ROOT/
    # https://thrift.apache.org/docs/install/debian.html    
    ./bootstrap.sh
    ./configure --with-qt5=no --with-kotlin=no --enable-tests=no
    make -j
    sudo make install
}

if [ "$#" -ne 1 ]; then
    echo "USAGE: $0 APACHE_THRIFT_VERSION"
    exit 1
fi

# pip install setuptools

build_thrift $1
echo "done."

exit 0
