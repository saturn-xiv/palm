#!/bin/bash

set -e

export GRPC_HOST="127.0.0.1:11004"

if [[ "$#" -eq 4 && "$1" == "create-user-by-email" ]]
then    
    grpcurl -plaintext -d '{"name": "'$2'", "email": "'$3'", "password": "'$4'"}' $GRPC_HOST palm.portal.v1.AdministratorService.CreateUserByEmail
elif [[ "$#" -eq 3 && "$1" == "set-password-for-email-user" ]]
then    
    grpcurl -plaintext -d '{"email": "'$2'", "password": "'$3'"}' $GRPC_HOST palm.portal.v1.AdministratorService.SetPasswordForEmailUser
elif [[ "$#" -eq 1 && "$1" == "list-user" ]]
then
    grpcurl -plaintext $GRPC_HOST palm.portal.v1.AdministratorService.ListUser
elif [[ "$#" -eq 1 && "$1" == "db-seeds" ]]
then
    grpcurl -plaintext $GRPC_HOST palm.portal.v1.AdministratorService.DbSeeds
elif [[ "$#" -eq 1 && "$1" == "help" ]]
then
    grpcurl -plaintext $GRPC_HOST list
elif [[ "$#" -eq 2 && "$1" == "help" ]]
then
    grpcurl -plaintext $GRPC_HOST list $2
elif [[ "$#" -eq 3 && "$1" == "help" ]]
then
    grpcurl -plaintext $GRPC_HOST describe "$2.$3"
else
    echo "Usage:
$0 help
$0 help SERVICE
$0 help SERVICE METHOD
$0 list-user
$0 db-seed
$0 create-user-by-email NAME EMAIL PASSWORD
$0 set-password-for-email-user EMAIL PASSWORD"
fi

exit 0
