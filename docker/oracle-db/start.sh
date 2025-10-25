#!/bin/bash

set -e

export CODE="container-registry.oracle.com/database/free:latest"
export NAME="palm-oracle-$1"

if [ "$#" -ne 2 ]; then
    echo "USAGE: $0 PORT DATA_DIR "
    exit 0
fi

if [ ! -d $2 ]; then
    mkdir -p $2
    chmod 777 $2
fi

if podman container exists $NAME; then
    podman start $NAME
else
    podman run -d --name $NAME --events-backend=file -p $1:1521 \
        -e ORACLE_PWD=change-me -e ORACLE_CHARACTERSET=AL32UTF8 -v $2:/opt/oracle/oradata \
        $CODE
fi

exit 0
