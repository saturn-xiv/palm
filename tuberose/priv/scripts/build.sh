#!/bin/bash

set -e

export MIX_ENV=prod
export PACKAGE_NAME=tuberose-$(date "+%4Y%m%d%H%M%S")

if [ -d _build ]; then
    rm -r _build
fi

mix deps.get --only prod
mix compile
MIX_ENV=prod mix assets.deploy
mix phx.gen.release
MIX_ENV=prod mix release

mkdir -p tmp
echo "build tmp/$PACKAGE_NAME.tar.xz ..."
tar -cf tmp/$PACKAGE_NAME.tar.xz -C _build/prod/rel/tuberose .
md5sum tmp/$PACKAGE_NAME.tar.xz >tmp/$PACKAGE_NAME.md5

echo 'done.'

exit 0
