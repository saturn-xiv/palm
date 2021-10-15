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
    # "cpr"
    "hiredis[ssl]"
    "zeromq[core,draft]"
    "cppzmq"
    "libpq[core]"
    # "libpqxx"
    "libmariadb[openssl]"
    "sqlite3"
    "sqlitecpp"
    "tomlplusplus"
    "nlohmann-json"
    "yaml-cpp"
    "tinyxml2"
    "cppcodec"
    "jwt-cpp"
    "inja"
    "cpp-httplib"
    "librabbitmq"
    "paho-mqtt"
    # "aws-sdk-cpp[core,s3]"
    "flatbuffers"
    # "grpc"
)

declare -a triplets=(
    "amd64-linux"
    "armhf-linux"
    # "amd64-linux"
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
