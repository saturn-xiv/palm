#!/bin/bash

set -e

source $HOME/local/python3/bin/activate
source /workspace/.env

pproxy -a 30 --reuse -l "$PROXY_URI" -r "ssh://$SSH_HOST:$SSH_PORT#$SSH_USER::/workspace/$SSH_KEYFILE"
