#!/bin/bash

export CODE="palm-marguerite"
export NAME="$CODE-$USER"

podman run --rm -it --events-backend=file --hostname=palm --network host -v $PWD:/workspace:z $CODE
