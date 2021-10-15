#!/bin/bash

set -e

declare -a packages=(
    "openssl"
    "boost"
    "curl[openssl]"
    "cpr"
    "hiredis[ssl]"
    "czmq[core]"
    "libpq[nls,uuid,openssl]"
    "libpqxx"
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
    "aws-sdk-cpp[s3]"
    "flatbuffers"
    # "grpc"
)

declare -a triplets=(
    "x64-linux"
    "arm-linux"
    "arm64-linux"
)

for p in "${packages[@]}"
do
    for t in "${triplets[@]}"
    do
        $HOME/local/vcpkg/vcpkg install --host-triplet=x64-linux --triplet=$t $p
    done
done

echo 'done.'
exit 0
