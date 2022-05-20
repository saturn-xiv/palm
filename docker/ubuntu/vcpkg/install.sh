#!/bin/bash

set -e

declare -a triples=(
    # "amd64-jammy"
    # "arm64-jammy"
    # "armhf-jammy"
    "x64-linux"
    "arm64-linux"
    "arm-linux"
)

declare -a packages=(
    "boost-algorithm"
    "boost-asio"
    "boost-beast"
    "boost-chrono"
    "boost-crc"
    "boost-date-time"
    "boost-filesystem"
    "boost-foreach"
    "boost-format"
    "boost-json"
    "boost-locale"
    "boost-log"
    "boost-math"
    "boost-program-options"
    "boost-property-tree"
    "boost-random"
    "boost-system"
    "boost-test"
    "boost-uuid"
    "zeromq[draft]"    
    "paho-mqtt"
    "librabbitmq"
    "mailio"
    "tomlplusplus"
    "yaml-cpp"
    "cppcodec"
    "nlohmann-json"
    "inja"
    "cpp-httplib"
    "cpr[core,ssl]"
    "curl[core,openssl,http2]"
    "protobuf"
    "grpc[core,absl-sync]"
    "libmariadb[core,openssl]"
    "libpq[core,openssl]"
    "sqlite3[core]"
    "soci[boost,sqlite3,postgresql]"
    # "poco[sqlite3,mariadb,postgresql]" #pdf mariadb, postgresql
)

for t in "${triples[@]}"
do
    for p in "${packages[@]}"
    do
        $HOME/local/vcpkg/vcpkg install $p --triplet=$t --host-triplet=x64-linux --recurse
    done
done

echo 'done.'

exit 0
