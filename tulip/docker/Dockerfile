FROM ubuntu:latest
LABEL maintainer="Jeremy Zheng"

ENV DEBIAN_FRONTEND noninteractive

RUN apt update
RUN apt -y upgrade
RUN apt -y install curl locales locales-all tzdata vim zip unzip \
    git clang cmake ninja-build pkg-config libtool automake autoconf autoconf-archive bison flex meson cpio \
    build-essential crossbuild-essential-amd64 crossbuild-essential-arm64 crossbuild-essential-riscv64

RUN apt -y autoremove
RUN apt -y clean

# https://learn.microsoft.com/en-us/vcpkg/get_started/get-started?pivots=shell-bash
RUN echo 'export VCPKG_DISABLE_METRICS=1' >> $HOME/.profile

# https://xmake.io/guide/quick-start.html
RUN curl -fsSL https://xmake.io/shget.text | bash
RUN echo 'export XMAKE_ROOT=y' >> $HOME/.profile

VOLUME /workspace
WORKDIR /workspace

CMD ["/bin/bash", "-l"]
