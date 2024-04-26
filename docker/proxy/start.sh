#!/bin/bash

export CODE="palm-proxy"
export NAME="$CODE-$USER"

podman run --rm --events-backend=file --hostname=palm --network host -v $PWD:/workspace:z $CODE
