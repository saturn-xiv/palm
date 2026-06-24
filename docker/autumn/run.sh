#!/bin/bash

export CODE="palm-autumn"

docker run --rm -it --network host -u $(id -u):$(id -g) -v $PWD:/srv:z $CODE
