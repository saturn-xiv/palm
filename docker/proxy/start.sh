#!/bin/bash

export CODE="palm-proxy"

podman run -d --rm --events-backend=file --hostname=palm --network host -v $PWD:/workspace:z $CODE
