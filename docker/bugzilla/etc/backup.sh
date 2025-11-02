#!/bin/bash

set -e

export VERSION=$(date "+%4Y%m%d%H%M%S")

mkdir -p $HOME/bugzilla-$VERSION
cd $HOME/bugzilla-$VERSION
cp -r /var/www/bugzilla repo
cp /etc/bugzilla/setup.pl /etc/apache2/sites-available/bugzilla.conf ./

cd $HOME/
XZ_OPT=-9 tar -cJf bugzilla-$VERSION.tar.xz bugzilla-$VERSION
mv bugzilla-$VERSION.tar.xz /workspace/

echo "done($VERSION)."
exit 0
