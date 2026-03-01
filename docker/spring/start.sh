#!/bin/bash

export CODE="palm-spring"
export NAME="$CODE-$USER"

if docker container exists $NAME; then
    docker start -i -a $NAME
else
    docker run --name $NAME -it --events-backend=file --hostname=palm --network host -v $PWD:/mnt:z $CODE
fi

