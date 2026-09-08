#!/bin/bash

set -e

set -o allexport
source .env
set +o allexport

function graphql_call() {
    curl -v -X POST -H "Content-Type: application/json" -H "Authorization: Bearer $AUTH_TOKEN" -d $1 $API_HOST
}

if [ "$#" -eq 1 && "$1" == "generate-headlam-token" ]; then
    graphql_call '{
    "query": "query call($hours: Int!) { lavender_k8s_generate_headlamp_token(hours: $hours){} }",
    "variables": {"hours": 8}
}'
if [ "$#" -eq 3 && "$1" == "run-job" ]; then
    graphql_call '{
    "query": "mutation call($id: String!, $args: [String!]!){ lavender_launch_job(id: $id, args: $args){createdAt} }",
    "variables": {"id": "'$2'", "args": ["'$3'"]}
}'
else
    echo "Unsupported $@"
    exit 1
fi
exit 0
