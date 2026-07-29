#!/bin/bash

export CODE="palm-plone"
export NAME="$CODE-$USER"

podman run --rm -it --events-backend=file --hostname=palm --network host -v $PWD:/workspace:z $CODE
