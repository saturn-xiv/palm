#!/bin/bash

set -e

export WORK_DIR=$PWD

cd $WORK_DIR/bougainvillea/
mix format

exit 0
