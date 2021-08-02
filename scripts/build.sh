#!/bin/sh

set -e
# ---------------------------------
USAGE=$(cat <<-END 
Please specify your arch!

Usage: $0 armhf|amd64
END
)

if [ $# -ne 1 ]
then
    echo $USAGE
    exit 1
fi
# ---------------------------------
export WORKSPACE=$PWD
export VERSION=$(git describe --tags --always --dirty --first-parent)
export TARGET=$WORKSPACE/tmp/palm-$1-$VERSION/target
# ---------------------------------
prepare_build() {
    if [ -f $TARGET/../palm_*.deb ]
    then
        echo "ingore..."
        exit 0
    fi

    if [ -d $TARGET ]
    then
        rm -rf $TARGET
    fi
    mkdir -pv $TARGET/usr/bin
    cp -r $WORKSPACE/debian $TARGET/
}

build_backend() {
    export PKG_CONpalm_ALL_STATIC=1
    # export SQLITE3_STATIC=1
    # FIXME
    # export PQ_LIB_STATIC=1 
    if [ "$1" = "amd64" ]
    then
        sudo apt -y install libudev-dev \
            libpq-dev libmysqlclient-dev libsqlite3-dev
        
        local target="x86_64-unknown-linux-gnu"
        cargo build --target $target --release
        cp -av $WORKSPACE/target/$target/release/palm $TARGET/usr/bin/
        strip -s $TARGET/usr/bin/palm
    elif [ "$1" = "armhf" ]
    then
        sudo apt -y install libc6-dev:armhf libudev-dev:armhf \
            libpq-dev:armhf libmysqlclient-dev:armhf libsqlite3-dev:armhf

        PKG_CONFIG_ALLOW_CROSS=1
        PKG_CONFIG_DIR=
        PKG_CONFIG_LIBDIR=/usr/lib/arm-linux-gnueabihf/pkgconfig
        export PKG_CONFIG_ALLOW_CROSS PKG_CONFIG_DIR PKG_CONFIG_LIBDIR

        # fix in dpkg-architecture
        CC=arm-linux-gnueabihf-gcc
        CXX=arm-linux-gnueabihf-g++
        export CC CXX

        local target="armv7-unknown-linux-gnueabihf"
        
        cargo build --target $target --release
        cp -av target/$target/release/palm $TARGET/usr/bin/
        arm-linux-gnueabihf-strip -s target/$target/release/palm
    else
        echo "unknown arch $1"
        exit 1
    fi
}

build_frontend(){
    if [ ! -d $WORKSPACE/node_modules ]
    then
        cd $WORKSPACE
        yarn install
    fi

    if [ ! -d $WORKSPACE/dashboard/node_modules ]
    then
        cd $WORKSPACE/dashboard
        yarn install
    fi
    cd $WORKSPACE/dashboard
    yarn build

    mkdir -pv $TARGET/usr/share/palm
    cp -av $WORKSPACE/node_modules $TARGET/usr/share/palm/
    cp -av $WORKSPACE/dashboard/build $TARGET/usr/share/palm/dashboard
}

build_deb() {
    mkdir -pv $TARGET/etc/palm
    cp -r $WORKSPACE/LICENSE $WORKSPACE/README.md $WORKSPACE/package.json $TARGET/etc/palm/
    echo "$VERSION $(date -R)" > $TARGET/etc/palm/VERSION

    mkdir -pv $TARGET/var/lib/palm

    mkdir -pv $TARGET/lib/systemd/system/

    cd $TARGET
    dpkg-buildpackage -us -uc -b --host-arch $1
}
# ---------------------------------

prepare_build
build_backend $1
build_frontend $1
build_deb $1

echo "Done($TARGET)."

exit 0
