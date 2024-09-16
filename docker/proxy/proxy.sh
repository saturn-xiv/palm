#!/bin/bash

set -e

source $HOME/local/python3/bin/activate
source /workspace/.env

pproxy --reuse -l "$PROXY_URI" -r "ssh://$SSH_HOST:$SSH_PORT#$SSH_USER::/workspace/$SSH_KEYFILE"
