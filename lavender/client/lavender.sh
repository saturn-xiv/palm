#!/bin/bash

set -e

set -o allexport
source .env
set +o allexport

function graphql_call() {
    curl -4 -X POST "$LAVENDER_HOST/graphql" -H "Content-Type: application/json; charset=utf-8" -H "Authorization: Bearer $LAVENDER_AUTH_TOKEN" -d "$1"
}

if [[ "$#" -eq 1 && "$1" == "generaqte-headlamp-token" ]]; then
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
elif [[ "$#" -eq 1 && "$1" == "export-redis-keys" ]]; then
    graphql_call '
{
    "query": "mutation call($id: String!, $args: [String!]!){ lavenderLaunchJob(id: $id, args: $args){createdAt} }",
    "variables": {"id": "'export-redis-keys'", "args": []}
}'
elif [[ "$#" -eq 1 && "$1" == "export-pali-synonyms" ]]; then
    graphql_call '
{
    "query": "mutation call($id: String!, $args: [String!]!){ lavenderLaunchJob(id: $id, args: $args){createdAt} }",
    "variables": {"id": "'export-pali-synonyms'", "args": []}
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
    echo "USAGE:
$0 version
$0 echo MESSAGE

$0 export-pali-synonyms
$0 export-redis-keys
$0 generaqte-headlamp-token DOMAIN GIT_COMMIT_ID"
    exit 1
fi
exit 0
