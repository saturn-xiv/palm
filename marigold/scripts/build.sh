#!/bin/bash

set -e

export WORK_DIR=$PWD
export PACKAGE=palm_"$(git describe --tags --always --dirty)"
export TARGET_DIR=$WORK_DIR/tmp

# -----------------------------------------------------------------------------

function build_python3() {
    local python_home=$WORK_DIR/tmp/python3
    if [ ! -d $python_home ]
    then
        python3 -m venv $python_home
    fi
    source $python_home/bin/activate
    pip install build

    cd $WORK_DIR/$1/
    if [ -d dist ]
    then
        rm -r dist
    fi
    python3 -m build

    mkdir -p $TARGET_DIR/$PACKAGE/$1
    cp README.md dist/$1-*-py3-none-any.whl staging.toml $TARGET_DIR/$PACKAGE/$1/

    deactivate
}

function build_java() {
    cd $WORK_DIR/$1/

    mvn clean
    mvn package -Dmaven.test.skip=true

    mkdir -p $TARGET_DIR/$PACKAGE/$1
    cp README.md staging.yml target/$1-*.jar $TARGET_DIR/$PACKAGE/$1/
}

function build_dashboard() {
    cd $WORK_DIR/$1/dashboard/

    if [ ! -d node_modules ]
    then
        npm install
    fi
    if [ -d dist ]
    then
        rm -r dist
    fi
    npm run build

    mkdir -p $TARGET_DIR/$PACKAGE/$1
    cp -r dist $TARGET_DIR/$PACKAGE/$1/dashboard
}

# -----------------------------------------------------------------------------

build_python3 hydrangea
build_java marigold
build_dashboard wisteria

cd $WORK_DIR/
cp README.md LICENSE $TARGET_DIR/$PACKAGE/

cd $TARGET_DIR/
tar -cJf $PACKAGE.tar.xz --remove-files $PACKAGE
md5sum $PACKAGE.tar.xz > $PACKAGE.md5

echo "done($PACKAGE)."
exit 0
