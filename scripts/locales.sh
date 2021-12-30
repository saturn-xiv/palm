#!/bin/bash

set -e


export WORKSPACE=$PWD

declare -a languages=(
    "en_US"
    "zh_CN"
    "zh_TW"
)


declare -a domains=(
    "auth"
    "rbac"
    "nut"
    "forum"
    "erp"
)

for l in "${languages[@]}"
do
    for d in "${domains[@]}"
    do
        cd $WORKSPACE/locales/$l/LC_MESSAGES
        echo "found $d($l)"
        msgfmt --check --verbose --output-file $d.mo $d.po
    done
done
echo 'done.'
