#!/bin/bash

set -e

if [ "$#" -ne 1 ]; then
    echo "USAGE: $0 PORT"
    exit 0
fi

podman exec -it palm-oracle-db sqlplus sys@localhost:$1/FREE as sysdba
