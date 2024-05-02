#!/bin/bash

set -e

apt update
DEBIAN_FRONTEND=noninteractive apt install -y openjdk-21-jdk maven

mvn clean
mvn package -Dmaven.test.skip=true

echo 'done.'
exit 0
