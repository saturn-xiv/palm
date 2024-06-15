#!/bin/bash

export CODE="palm-dm8"
export NAME="$CODE-$USER-$1"

if [ "$#" -ne 1 ]; then
    echo "USAGE: $0 PORT"
    exit 0
fi

if podman container exists $NAME; then
    podman start -i -a $NAME
else
    podman run --name $NAME -it --events-backend=file -v $PWD:/workspace:z -p $1:5236/tcp $CODE
fi
