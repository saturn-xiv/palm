#!/bin/bash

source $HOME/local/python3/bin/activate
source /workspace/.env

pproxy --reuse -l "http://:$LOCAL_PORT" -r "ssh://$SSH_HOST:$SSH_PORT#$SSH_USER::/workspace/$SSH_KEYFILE"
