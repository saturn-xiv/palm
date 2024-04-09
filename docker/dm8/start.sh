#!/bin/bash

export CODE="palm-dm8"
export NAME="$CODE-$USER"

if podman container exists $NAME; then
    podman start -i -a $NAME
else
    #  –v /sys/fs/cgroup:/sys/fs/cgroup:ro --systemd=true
    # podman run --name $NAME -it --hostname=palm --network host --privileged -v $PWD:/workspace:z $CODE /sbin/init
    podman run --name $NAME -it --events-backend=file --hostname=palm --network host -v $PWD:/workspace:z $CODE
fi
