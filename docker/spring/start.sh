#!/bin/bash

export CODE="palm-spring"
export NAME="spring"
export WORK_DIR="$PWD/mnt"


if [ ! -f $WORK_DIR/authorized_keys ]
then
    echo "Couldn't found authorized_keys file"
    exit 1
fi

if [ "$(docker inspect -f '{{.State.Status}}' $NAME)" = "running" ]; then
    echo "container $NAME is active!"
    exit 0
fi

if [ -n "$(docker ps -a -q -f name="^${NAME}$")" ] ; then
    docker start $NAME
    exit 0
fi

docker run -d --name $NAME --hostname=palm --network host -v $WORK_DIR:/mnt:z $CODE /mnt/start.sh
