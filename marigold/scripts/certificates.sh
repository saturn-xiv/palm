#!/bin/bash

set -e

if [ "$#" -ne 3 ]; then
    echo "Usage: $0 <domain> <name> <days>"
    exit 1
fi

export WORK_DIR=$PWD/$1

mkdir -p $WORK_DIR
cd $WORK_DIR/
if [ ! -f ca.key ]
then
    echo "Generate CA private key and self-signed certificate"
    openssl genrsa -out ca.key 4096
    openssl req -x509 -new -nodes -key ca.key -sha256 -days $3 -out ca.crt -subj "/CN=$1"
fi

if [ ! -f ${2}.key ]
then
    echo "Generate ${2}.${1} private key and certificate signing request (CSR)"
    openssl genrsa -out ${2}.key 4096
    openssl req -new -key ${2}.key -out ${2}.csr -subj "/CN=${2}.${1}"
    echo "subjectAltName = DNS:localhost,IP:127.0.0.1" > ${2}.ext
    openssl x509 -req -in ${2}.csr -CA ca.crt -CAkey ca.key -CAcreateserial -out ${2}.crt -days $3 -sha512 -extfile ${2}.ext
fi

echo "done(${2}.${1})."
exit 0
