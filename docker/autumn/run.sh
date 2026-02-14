#!/bin/bash

export CODE="palm-autumn"

docker run --rm -it --network host -v $PWD:/srv:z $CODE
