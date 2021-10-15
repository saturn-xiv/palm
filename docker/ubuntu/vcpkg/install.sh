#!/bin/bash

set -e

declare -a packages=(
    "openssl"
    "boost-algorithm"
    "boost-asio"
    "boost-beast"
    "boost-chrono"
    "boost-crc"
    "boost-date-time"
    "boost-format"
    "boost-log"
    "boost-math"
    "boost-pool"
    "boost-program-options"
    "boost-property-tree"
    "boost-random"
    "boost-test"
    "boost-timer"
    "boost-uuid"
    "curl[core,openssl]"
    "hiredis[ssl]"
    "zeromq[core,draft]"
    "cppzmq"
    "libmariadb[core,openssl]"
    "sqlite3"
    "sqlitecpp"
    "nlohmann-json"
    "yaml-cpp"
    "tinyxml2"
    "cppcodec"
    "jwt-cpp"
    "inja"
    "cpp-httplib"
    "librabbitmq"
    "paho-mqtt"
    "flatbuffers"

    # "tomlplusplus" # arm
    "libpq[core,client]"
    # "libpqxx"
    # "cpr" # cross build
    # "aws-sdk-cpp[core,s3]"
    # "grpc" # old version

)

declare -a triplets=(
    "amd64-linux"
    "armhf-linux"
    "arm64-linux"
)

for p in "${packages[@]}"
do
    for t in "${triplets[@]}"
    do
        $HOME/local/vcpkg/vcpkg install --recurse --host-triplet=x64-linux --triplet=$t $p
    done
done

echo 'done.'
exit 0
