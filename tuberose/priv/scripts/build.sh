#!/bin/bash

set -e

export MIX_ENV=prod
export PACKAGE_NAME=tuberose-$(date "+%4Y%m%d%H%M%S")

mix deps.get
mix compile
MIX_ENV=prod mix assets.deploy
mix phx.gen.release
MIX_ENV=prod mix release

mkdir -p tmp
tar -cf tmp/$PACKAGE_NAME.tar.xz -C _build/prod/rel _build/prod/rel/tuberose
md5sum tmp/$PACKAGE_NAME.tar.xz >tmp/$PACKAGE_NAME.md5

echo "done($PACKAGE_NAME.tar.xz)."

exit 0
