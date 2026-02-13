FROM ubuntu:noble
LABEL maintainer="Jeremy Zheng"

ENV DEBIAN_FRONTEND noninteractive

RUN apt update
RUN apt -y upgrade
RUN apt -y install curl locales locales-all tzdata vim zip unzip \
    git clang cmake ninja-build pkg-config libtool automake autoconf autoconf-archive bison flex meson cpio \
    build-essential crossbuild-essential-arm64

RUN apt -y autoremove
RUN apt -y clean

RUN echo "en_US.UTF-8 UTF-8" > /etc/locale.gen
RUN locale-gen
RUN update-locale LANG=en_US.UTF-8
RUN update-alternatives --set editor /usr/bin/vim.basic

# https://learn.microsoft.com/en-us/vcpkg/get_started/get-started?pivots=shell-bash
RUN echo 'export VCPKG_DISABLE_METRICS=1' >> $HOME/.profile

# https://xmake.io/guide/quick-start.html
RUN curl -fsSL https://xmake.io/shget.text | bash
RUN echo 'export XMAKE_ROOT=y' >> $HOME/.profile

VOLUME /workspace
WORKDIR /workspace

CMD ["/bin/bash", "-l"]
