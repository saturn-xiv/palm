#!/bin/bash

set -e


export WORK_DIR=$PWD
export VCPKG_PACKAGES_HOME=$WORK_DIR/loquat/vcpkg/packages
export PYTHON_HOME=$(python3 -c "import sys; import os; print(os.path.dirname(os.path.dirname(sys.executable)))")

# -----------------------------------------------------------------------------

function generate_hydrangea() {
    echo "generate hydrangea protocols"
    cd $WORK_DIR/hydrangea/src/
    rm -f hydrangea/protocols/*_pb2*
    
    python -m grpc_tools.protoc -I $VCPKG_PACKAGES_HOME/protobuf_x64-linux-release/include/google/protobuf \
        -Ihydrangea/protocols=$WORK_DIR/marigold/src/main/proto \
        --python_out=. --pyi_out=. --grpc_python_out=. \
        $WORK_DIR/marigold/src/main/proto/*.proto
}

function format_sources() {
    echo "format hydrangea"
    cd $WORK_DIR/hydrangea/
    autopep8 --in-place --recursive src --exclude="src/hydrangea/protocols/*"
}

# -----------------------------------------------------------------------------

if [ ! -f $VCPKG_PACKAGES_HOME/protobuf_x64-linux-release/tools/protobuf/protoc ]
then
    echo "Couldn't found protoc command"
    exit 1
fi

if [ ! -f $PYTHON_HOME/bin/python-grpc-tools-protoc ]
then
    echo "Couldn't found protoc plugin for Python: pip install 'grpcio-tools~=1.82'"
    exit 1
fi


generate_hydrangea
format_sources

echo 'done.'
exit 0
