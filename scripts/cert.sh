#!/bin/bash

set -e

function generate_mutual() {
    if [ ! -f ca.key ]; then
        echo "Generate CA's private key and self-signed certificate"
        openssl req -x509 -newkey rsa:4096 -days $2 -nodes -keyout ca.key -out ca.crt -subj "/C=US/ST=California/L=San Francisco/O=Saturn XIV/OU=Ops/CN=*.saturn-xiv.org/emailAddress=ops@saturn-xiv.org"

        echo "CA's self-signed certificate"
        openssl x509 -in ca.crt -noout -text
    fi

    if [ ! -f $1.key ]; then
        if [ ! -f $1.cnf ]; then
            echo "Counldn't found $1.cnf: subjectAltName=DNS:*.saturn-xiv.org,DNS:*.saturn-xiv.edu,IP:0.0.0.0"
            exit 1
        fi
        echo "Generate a private key and certificate signing request (CSR)"
        openssl req -newkey rsa:4096 -nodes -keyout $1.key -out $1.req -subj "/C=US/ST=California/L=San Francisco/O=Saturn XIV/OU=Www/CN=*.saturn-xiv.org/emailAddress=www@saturn-xiv.org"

        echo "Use CA's private key to sign a CSR and get back the signed certificate"
        openssl x509 -req -in $1.req -days $2 -CA ca.crt -CAkey ca.key -CAcreateserial -out $1.crt -extfile $1.cnf

        echo "It's signed certificate"
        openssl x509 -in $1.crt -noout -text
    fi

}

if [ "$#" -ne 2 ]; then
    echo "USAGE: $0 NAME DAYS"
    exit 1
fi

generate_mutual $1 $2

echo 'done.'
exit 0
