#!/bin/bash

set -e

set -o allexport
source .env
set +o allexport

function graphql_call() {
    curl -4 -X POST "$LAVENDER_HOST/graphql" -H "Content-Type: application/json; charset=utf-8" -H "Authorization: Bearer $LAVENDER_AUTH_TOKEN" -d "$1"
}

if [[ "$#" -eq 1 && "$1" == "headlamp-token-generate" ]]; then
    graphql_call '
{
    "query": "mutation call($name: String!, $args: [String!]!){ lavenderLaunchJob(name: $name, args: $args){createdAt} }",
    "variables": {"name": "'headlamp-token-generate'", "args": []}
}'
elif [[ "$#" -eq 3 && "$1" == "mint-deployment" ]]; then
    graphql_call '
{
    "query": "mutation call($name: String!, $args: [String!]!){ lavenderLaunchJob(name: $name, args: $args){createdAt} }",
    "variables": {"name": "'mint-deployment'", "args": ["'$2'", "'$3'"]}
}'
elif [[ "$#" -eq 1 && "$1" == "redis-keys-export" ]]; then
    graphql_call '
{
    "query": "mutation call($name: String!, $args: [String!]!){ lavenderLaunchJob(name: $name, args: $args){createdAt} }",
    "variables": {"name": "'export-redis-keys'", "args": []}
}'
elif [[ "$#" -eq 1 && "$1" == "pali-synonyms-export" ]]; then
    graphql_call '
{
    "query": "mutation call($name: String!, $args: [String!]!){ lavenderLaunchJob(name: $name, args: $args){createdAt} }",
    "variables": {"name": "'pali-synonyms-export'", "args": []}
}'
elif [[ "$#" -eq 2 && "$1" == "echo" ]]; then
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
$0 echo MESSAGE

$0 pali-synonyms-export
$0 redis-keys-export
$0 headlamp-token-generate DOMAIN GIT_COMMIT_ID"
    exit 1
fi
exit 0
