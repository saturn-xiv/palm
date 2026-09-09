#!/bin/bash

export CODE="palm-spring"

docker run --rm -it --hostname=palm --network host -v $PWD:/mnt:z $CODE /usr/bin/zsh -l
