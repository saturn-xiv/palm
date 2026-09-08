#!/bin/bash

set -e

# https://github.com/TEIC/Stylesheets
export TEIC_HOME=$HOME/workspace/TEIC/Stylesheets
export WORK_DIR=$PWD
export TARGET_DIR=$WORK_DIR/tmp/epub

cd $WORK_DIR/vendors/tipitaka-xml/romn/
mkdir -p $WORK_DIR/tmp/epub/tipitaka-romn
for file in *.xml; do
    echo "processing $file => ${file%.*}.epub"
    $TEIC_HOME/bin/teitoepub3 $file $WORK_DIR/tmp/epub/tipitaka-romn/${file%.*}.epub
done

echo 'done.'
exit 0
