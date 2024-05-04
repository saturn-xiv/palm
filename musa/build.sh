#!/bin/bash

set -e

apt update
DEBIAN_FRONTEND=noninteractive apt install -y openjdk-21-jdk gradle

# mvn clean
# mvn package -Dmaven.test.skip=true

gradle clean
gradle build

echo 'done.'
exit 0
