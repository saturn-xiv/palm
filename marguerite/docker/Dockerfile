FROM ubuntu:jammy
LABEL maintainer="Jeremy Zheng"

ENV DEBIAN_FRONTEND noninteractive

RUN apt update
RUN apt install -y lsb-release apt-utils \
    debian-keyring debian-archive-keyring apt-transport-https software-properties-common curl wget gnupg
# https://apt.llvm.org/
ENV CLANG_VERSION 18
RUN echo "deb http://apt.llvm.org/$(lsb_release -cs)/ llvm-toolchain-$(lsb_release -cs)-${CLANG_VERSION} main" > /etc/apt/sources.list.d/llvm.list
RUN wget -qO- https://apt.llvm.org/llvm-snapshot.gpg.key | tee /etc/apt/trusted.gpg.d/apt.llvm.org.asc
# https://wiki.debian.org/ToolChain/Cross
RUN add-apt-repository -y ppa:ubuntu-toolchain-r/test
RUN add-apt-repository -y ppa:rabbitmq/rabbitmq-erlang
RUN apt update
RUN apt -y upgrade
RUN apt install -y git zip \
    build-essential pkg-config cmake flex bison ninja-build autoconf libtool mold \
    g++-13 \
    clang-${CLANG_VERSION} clangd-${CLANG_VERSION} clang-tools-${CLANG_VERSION} clang-format-${CLANG_VERSION} clang-tidy-${CLANG_VERSION} lldb-${CLANG_VERSION} lld-${CLANG_VERSION} \
    linux-libc-dev \
    erlang elixir inotify-tools

RUN echo "$(date -u +%4Y%m%d%H%M%S)" | tee /VERSION

VOLUME /workspace
WORKDIR /workspace

CMD ["/bin/bash", "-l"]
