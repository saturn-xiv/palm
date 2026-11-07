#!/bin/bash

set -e

export WORK_DIR=$PWD
export VCPKG_PACKAGES_HOME=$WORK_DIR/loquat/vcpkg/packages
export ELIXIR_HOME=$(elixir -e 'IO.puts(:code.lib_dir(:elixir)|>Path.dirname|>Path.dirname|>Path.dirname|>Path.dirname)')
export PYTHON_HOME=$(python3 -c "import sys; import os; print(os.path.dirname(os.path.dirname(sys.executable)))")
export PATH=$ELIXIR_HOME/.mix/escripts:$PATH

# -----------------------------------------------------------------------------

function generate_loquat() {
    echo "generate loquat protocols"
    rm -r $WORK_DIR/loquat/gourd/src $WORK_DIR/loquat/gourd/include
    mkdir $WORK_DIR/loquat/gourd/src $WORK_DIR/loquat/gourd/include

    $VCPKG_PACKAGES_HOME/protobuf_x64-linux-release/tools/protobuf/protoc -I $VCPKG_PACKAGES_HOME/protobuf_x64-linux-release/include/google/protobuf \
        -I $WORK_DIR/loquat/proto \
        --cpp_out=$WORK_DIR/loquat/gourd/src --grpc_out=$WORK_DIR/loquat/gourd/src \
        --plugin=protoc-gen-grpc=$VCPKG_PACKAGES_HOME/grpc_x64-linux/tools/grpc/grpc_cpp_plugin \
        $WORK_DIR/loquat/proto/*.proto
    mv $WORK_DIR/loquat/gourd/src/*.h $WORK_DIR/loquat/gourd/include/
}

function generate_dahlia() {
    echo "generate dahlia protocols"
    cd $WORK_DIR/dahlia/src/
    rm -f dahlia/protocols/*_pb2*

    python -m grpc_tools.protoc -I $VCPKG_PACKAGES_HOME/protobuf_x64-linux-release/include/google/protobuf \
        -Idahlia/protocols=$WORK_DIR/dahlia/proto \
        --python_out=. --pyi_out=. --grpc_python_out=. \
        $WORK_DIR/dahlia/proto/*.proto
}

function generate_hydrangea() {
    echo "generate hydrangea protocols"
    cd $WORK_DIR/hydrangea/src/
    rm -f hydrangea/protocols/*_pb2*

    python -m grpc_tools.protoc -I $VCPKG_PACKAGES_HOME/protobuf_x64-linux-release/include/google/protobuf \
        -Ihydrangea/protocols=$WORK_DIR/bougainvillea/priv/proto \
        --python_out=. --pyi_out=. --grpc_python_out=. \
        $WORK_DIR/bougainvillea/priv/proto/tex.proto \
        $WORK_DIR/bougainvillea/priv/proto/cups.proto \
        $WORK_DIR/bougainvillea/priv/proto/hydrangea.proto
    python -m grpc_tools.protoc -I $VCPKG_PACKAGES_HOME/protobuf_x64-linux-release/include/google/protobuf \
        -Ihydrangea/protocols=$WORK_DIR/marigold/src/main/proto \
        --python_out=. --pyi_out=. --grpc_python_out=. \
        $WORK_DIR/marigold/src/main/proto/email.proto $WORK_DIR/marigold/src/main/proto/sms.proto
}

function generate_bougainvillea() {
    echo "generate bougainvillea protocols"
    cd $WORK_DIR/bougainvillea/

    rm -r lib/palm
    local -a plugins=("loquat" "dahlia" "marigold/src/main" "bougainvillea/priv")
    for p in "${plugins[@]}"
    do
        $VCPKG_PACKAGES_HOME/protobuf_x64-linux-release/tools/protobuf/protoc -I $VCPKG_PACKAGES_HOME/protobuf_x64-linux-release/include/google/protobuf \
            --elixir_out=plugins=grpc:./lib \
            -I$WORK_DIR/$p/proto $WORK_DIR/$p/proto/*.proto
    done
}

function format_sources() {
    echo "format loquat"
    cd $WORK_DIR/loquat/
    clang-format -i include/loquat/*.hpp src/*.cpp

    echo "format dahlia"
    cd $WORK_DIR/dahlia/
    autopep8 --in-place --recursive src --exclude="src/dahlia/protocols/*"

    echo "format hydrangea"
    cd $WORK_DIR/hydrangea/
    autopep8 --in-place --recursive src --exclude="src/hydrangea/protocols/*"

    echo "format bougainvillea"
    cd $WORK_DIR/bougainvillea/
    mix format
}

# -----------------------------------------------------------------------------

if [ ! -f $VCPKG_PACKAGES_HOME/protobuf_x64-linux-release/tools/protobuf/protoc ]
then
    echo "Couldn't found protoc command"
    exit 1
fi

# https://github.com/elixir-protobuf/protobuf#usage
if [ ! -f $ELIXIR_HOME/.mix/escripts/protoc-gen-elixir ]
then
    echo "Couldn't found protoc plugin for Elixir: mix escript.install hex protobuf 0.17.1"
    exit 1
fi

if [ ! -f $PYTHON_HOME/bin/python-grpc-tools-protoc ]
then
    echo "Couldn't found protoc plugin for Python: pip install 'grpcio-tools~=1.82'"
    exit 1
fi

generate_loquat
generate_dahlia
generate_bougainvillea
generate_hydrangea

format_sources

echo 'done.'
exit 0
