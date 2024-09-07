#!/bin/bash

set -e

npm install --save @grpc/grpc-js grpc-health-check google-protobuf \
    marked dompurify jsdom canvas \
    pino pino-pretty \
    bufferutil utf-8-validate

npm install --save-dev webpack webpack-cli

echo 'done.'
exit 0
