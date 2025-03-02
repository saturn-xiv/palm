FROM ubuntu:latest
LABEL maintainer="Jeremy Zheng"

ENV DEBIAN_FRONTEND noninteractive

RUN apt update
RUN apt install -y lsb-release apt-utils \
    debian-keyring debian-archive-keyring apt-transport-https software-properties-common curl wget gnupg
RUN apt -y upgrade
RUN apt install -y git zip vim locales \
    pkg-config cmake flex bison ninja-build autoconf libtool mold \
    nodejs npm \
    python3-full python3-dev libssl-dev libmariadb-dev libhiredis-dev
RUN apt -y autoremove
RUN apt -y clean

# https://grpc.io/docs/protoc-installation/
ARG PROTOBUF_VERSION="25.6"
# aarch_64
ARG PROTOBUF_ARCH="x86_64" 
RUN wget -q -P $HOME/downloads \
    https://github.com/protocolbuffers/protobuf/releases/download/v${PROTOBUF_VERSION}/protoc-${PROTOBUF_VERSION}-linux-${PROTOBUF_ARCH}.zip
RUN unzip -q $HOME/downloads/protoc-${PROTOBUF_VERSION}-linux-${PROTOBUF_ARCH}.zip -d /usr/local

# https://github.com/bazelbuild/bazelisk
RUN npm install -g @bazel/bazelisk

RUN echo "en_US.UTF-8 UTF-8" > /etc/locale.gen
RUN locale-gen

RUN echo "$(date -u +%4Y%m%d%H%M%S)" | tee /VERSION

VOLUME /workspace
WORKDIR /workspace

CMD ["/bin/bash", "-l"]
