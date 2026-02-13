#!/bin/bash

export CODE="palm-autumn"

podman run --rm -it --events-backend=file --hostname=palm --network host -v $PWD:/workspace:z $CODE


