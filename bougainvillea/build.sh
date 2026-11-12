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

# https://phoenix.hexdocs.pm/deployment.html
function build_phoenix() {
    cd $WORK_DIR/$1/
        
    mix deps.get --only prod
    
    if [ -d _build/prod/rel/$1 ]
    then
        rm -r _build/prod/rel/$1
    fi

    export MIX_ENV=prod
    mix compile
    mix phx.digest.clean --all
    git clean -fd rel/ lib/$1/release.ex
    mix assets.deploy
    mix phx.gen.release
    mix release --overwrite
    
    cp -r _build/prod/rel/$1 $TARGET_DIR/$PACKAGE/
    mkdir -p $TARGET_DIR/$PACKAGE/$1/etc
    cp -r README.md .env.staging $TARGET_DIR/$PACKAGE/$1/etc/
    
    mix phx.digest.clean --all
    git clean -fd rel/ lib/$1/release.ex
}

function build_loquat() {
    cd $WORK_DIR/loquat/
    bash build.sh
    
    cd $WORK_DIR/loquat/
    local -a targets=("x86_64" "aarch64" "riscv64")
    for t in "${targets[@]}"
    do
        mkdir -p $TARGET_DIR/$PACKAGE/bin/$t
        cp build/$t/loquat $TARGET_DIR/$PACKAGE/bin/$t/
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

if [ -f $TARGET_DIR/$PACKAGE.md5 ]
then
    echo "building $PACKAGE already exists."
    exit 0
fi
if [ -d $TARGET_DIR/$PACKAGE ]
then
    rm -r $TARGET_DIR/$PACKAGE
fi

build_java marigold
build_python3 dahlia
build_python3 hydrangea
build_phoenix bougainvillea
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
