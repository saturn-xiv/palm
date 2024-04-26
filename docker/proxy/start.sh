#!/bin/bash

export CODE="palm-proxy"

podman run --rm --events-backend=file --hostname=palm --network host -v $PWD:/workspace:z $CODE
