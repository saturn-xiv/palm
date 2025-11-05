#!/bin/bash

set -e

export GO_HOME=$HOME/local/go
export GO_VERSION="1.25.3"

if [ ! -d $GO_HOME ]
then
    wget -q -P $HOME/downloads https://go.dev/dl/go${GO_VERSION}.linux-amd64.tar.gz
    tar xf $HOME/downloads/go${GO_VERSION}.linux-amd64.tar.gz -C $HOME/local
fi

export PATH=$GO_HOME/bin:$PATH
export GOPATH=$HOME/go
export PATH="$(go env GOPATH)/bin:$PATH"

# https://code.visualstudio.com/docs/languages/go
go install golang.org/x/tools/gopls@latest
go install github.com/go-delve/delve/cmd/dlv@latest
go install honnef.co/go/tools/cmd/staticcheck@latest

# https://grpc.io/docs/languages/go/quickstart/
go install google.golang.org/protobuf/cmd/protoc-gen-go@latest
go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest

go install github.com/fullstorydev/grpcurl/cmd/grpcurl@latest

echo 'done.'
