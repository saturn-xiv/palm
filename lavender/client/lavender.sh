#!/bin/bash

set -e

set -o allexport
source .env
set +o allexport

function graphql_call() {
    curl -4 -X POST "$LAVENDER_HOST/graphql" -H "Content-Type: application/json; charset=utf-8" -H "Authorization: Bearer $LAVENDER_AUTH_TOKEN" -d "$1"
}

if [[ "$#" -eq 1 && "$1" == "headlamp-token" ]]; then
    graphql_call '
{
    "query": "mutation call($id: String!, $args: [String!]!){ lavenderLaunchJob(id: $id, args: $args){createdAt} }",
    "variables": {"id": "'generate-headlamp-token'", "args": ["8"]}
}'
elif [[ "$#" -eq 3 && "$1" == "run-deployment" ]]; then
    graphql_call '
{
    "query": "mutation call($id: String!, $args: [String!]!){ lavenderLaunchJob(id: $id, args: $args){createdAt} }",
    "variables": {"id": "'deployment'", "args": ["'$2'", "'$3'"]}
}'
elif [[ "$#" -eq 2 && "$1" == "echo" ]]; then
    graphql_call '
{
    "query": "mutation call($id: String!, $args: [String!]!){ lavenderLaunchJob(id: $id, args: $args){createdAt} }",
    "variables": {"id": "'echo'", "args": ["'$2'"]}
}'
elif [[ "$#" -eq 1 && "$1" == "version" ]]; then
    graphql_call '
{
    "query": "query call{ apiVersion }",
    "variables": {}
}'
else
    echo "Unsupported $@"
    exit 1
fi
exit 0
