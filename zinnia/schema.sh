#!/bin/bash

set -e

function generate_for_zinnia() {
    python -m grpc_tools.protoc -Igrpc/example/custom/path=../../protos \
        --python_out=. --grpc_python_out=. \
        ../../protos/route_guide.proto
}

echo 'done.'
