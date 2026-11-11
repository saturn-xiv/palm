#!/bin/bash

set -e

source /etc/os-release

export WORK_DIR=$PWD
export PACKAGE=palm_"$(git describe --tags --always --dirty)-${VERSION_CODENAME}-$(uname -m)"
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

function build_elixir() {
    cd $WORK_DIR/$1/
    
    export MIX_ENV=prod
    mix deps.get --only prod
    mix clean
    mix compile
    mix release --overwrite

    mkdir -p $TARGET_DIR/$PACKAGE/$1
    cp -r _build/prod/rel/$1 $TARGET_DIR/$PACKAGE/
    cp -r README.md staging.toml $TARGET_DIR/$PACKAGE/$1/
}

function build_loquat() {
    cd $WORK_DIR/loquat/
    bash build.sh
    
    cd $WORK_DIR/loquat/
    local -a targets=("x86_64" "aarch64" "riscv64")
    for t in "${targets[@]}"
    do
        mkdir -p $TARGET_DIR/$PACKAGE/$t/bin
        cp build/$t/loquat $TARGET_DIR/$PACKAGE/$t/bin/
    done
}

function amazon_corretto_jdk() {
    local -a targets=("x64" "aarch64")
    for t in "${targets[@]}"
    do
        local f=amazon-corretto-27-${t}-linux-jdk.tar.gz
        if [ ! -f $WORK_DIR/tmp/$f ]
        then
            wget -P $WORK_DIR/tmp/ https://corretto.aws/downloads/latest/$f
        fi
        # cp $WORK_DIR/tmp/$f $TARGET_DIR/$PACKAGE/share/
    done
} 

# -----------------------------------------------------------------------------

build_java marigold
build_python3 dahlia
build_python3 hydrangea
build_elixir bougainvillea
build_loquat

mkdir -p $TARGET_DIR/$PACKAGE/share
amazon_corretto_jdk

cd $WORK_DIR/
cp README.md LICENSE $TARGET_DIR/$PACKAGE/

cd $TARGET_DIR/
tar -cJf $PACKAGE.tar.xz --remove-files $PACKAGE
md5sum $PACKAGE.tar.xz > $PACKAGE.md5

echo "done($PACKAGE.tar.xz)."
exit 0
