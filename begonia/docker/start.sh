#!/bin/bash

export CODE="begonia"

podman run --rm -it --events-backend=file --hostname=palm --network host -v $PWD:/workspace:z $CODE
