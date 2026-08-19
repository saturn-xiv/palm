#!/bin/bash

export CODE="palm-autumn"


if [ ! -d tmp ]
then
  mkdir tmp
  chmod 777 tmp
fi

docker run --rm -it --network host -v $PWD/tmp:/srv:z $CODE
# docker run --rm -it --network host -u $(id -u):$(id -g) -v $PWD/tmp:/srv:z $CODE
