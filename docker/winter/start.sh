#!/bin/bash

export CODE="palm-winter"
export NAME="$CODE-$USER"

if docker inspect $NAME >/dev/null 2>&1; then
    docker start -i -a $NAME
else
    docker run --name $NAME -it --network host -v $PWD:/workspace:z $CODE
fi
