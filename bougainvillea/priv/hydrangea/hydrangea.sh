#!/bin/bash

if [[ -f ".env" ]]; then
    set -a
    source .env
    set +a
fi 

set -e



# https://developers.cloudflare.com/analytics/graphql-api/getting-started/execute-graphql-query/
function graphql_call() {
    curl -v -4 -X POST "$HYDRANGEA_ENDPOINT/graphql" -H "Content-Type: application/json" -H "Authorization: Bearer $HYDRANGEA_AUTH_TOKEN" -d "$1"
}

if [[ "$#" -eq 2 && "$1" == "echo" ]]; then
    graphql_call '
{
    "query": "mutation call($name: String!, $args: [String!]!){ hydrangeaExecuteBackgroundJob(name: $name, args: $args){ createdAt } }",
    "variables": {"name": "echo", "args": ["'${2}'"]}
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
