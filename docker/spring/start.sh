#!/bin/bash

export CODE="palm-spring"
export NAME="$CODE-$USER"

if [ -n "$(docker ps -a -q -f name="^${NAME}$")" ] ; then
    docker start -i -a $NAME
else
    docker run --name $NAME -it --hostname=palm --network host -v $PWD:/mnt:z $CODE
fi

