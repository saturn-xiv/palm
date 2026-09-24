#!/bin/bash

set -e

set -o allexport
source .env
set +o allexport

function graphql_call() {
    curl -4 -X POST "$LAVENDER_HOST/graphql" -H "Content-Type: application/json; charset=utf-8" -H "Authorization: Bearer $LAVENDER_AUTH_TOKEN" -d "$1"
}

if [[ "$#" -eq 2 && "$1" == "echo" ]]; then
    graphql_call '
{
    "query": "mutation call($name: String!, $args: [String!]!){ lavenderLaunchJob(name: $name, args: $args){createdAt} }",
    "variables": {"name": "'echo'", "args": ["'$2'"]}
}'
elif [[ "$#" -eq 1 && "$1" == "version" ]]; then
    graphql_call '
{
    "query": "query call{ apiVersion }",
    "variables": {}
}'
else
    echo "USAGE:
$0 version
$0 echo MESSAGE"
    exit 1
fi
exit 0
