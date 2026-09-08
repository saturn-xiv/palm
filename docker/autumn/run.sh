#!/bin/bash

export CODE="palm-autumn"

docker run --rm -it --network host -v $PWD:/mnt:z $CODE
# docker run --rm -it --network host -u $(id -u):$(id -g) -v $PWD:/mnt:z $CODE
