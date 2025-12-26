#!/bin/bash

export CODE="palm-spring"
export NAME="$CODE-$USER"

# if podman container exists $NAME; then
#     podman start -i -a $NAME
# else
#     podman run --name $NAME -it --events-backend=file --hostname=palm --network host -v $PWD:/workspace:z $CODE
# fi


if docker container exists $NAME; then
    docker start -i -a $NAME
else
    docker run --name $NAME -it --hostname=palm --network host -v $PWD:/workspace:z $CODE
fi
