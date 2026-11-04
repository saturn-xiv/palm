#!/bin/bash

set -e

export WORK_DIR=$PWD
export PACKAGE=palm_"$(git describe --tags --always --dirty)-${VERSION_CODENAME}-$(uname -m)"
export TARGET_DIR=$WORK_DIR/tmp

cd $WORK_DIR/hibiscus/
mvn clean
mvn package -Dmaven.test.skip=true
mkdir -p $TARGET_DIR/$PACKAGE/hibiscus
cp README.md staging.yml target/hibiscus-*.jar $TARGET_DIR/$PACKAGE/hibiscus/

cd $WORK_DIR/bougainvillea/
export MIX_ENV=prod
mix deps.get --only prod
mix clean
mix compile
mix release --overwrite
cp -r _build/prod/rel/bougainvillea $TARGET_DIR/$PACKAGE/
cp README.md $TARGET_DIR/$PACKAGE/BOUGAINVILLEA.md

cd $WORK_DIR/
cp README.md LICENSE $TARGET_DIR/$PACKAGE/

cd $TARGET_DIR/
tar -cJf $PACKAGE.tar.xz --remove-files $PACKAGE
md5sum $PACKAGE.tar.xz > $PACKAGE.md5

echo "done($PACKAGE.tar.xz)."
exit 0
