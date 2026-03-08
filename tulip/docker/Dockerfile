FROM ubuntu:latest
LABEL maintainer="Jeremy Zheng"

ENV DEBIAN_FRONTEND noninteractive

RUN apt update
RUN apt -y upgrade
RUN apt -y install lsb-release wget software-properties-common gnupg \
    curl locales locales-all tzdata vim zip unzip \
    git cmake ninja-build pkg-config libtool automake autoconf autoconf-archive bison flex meson cpio mold \
    build-essential crossbuild-essential-amd64 crossbuild-essential-arm64 crossbuild-essential-riscv64

RUN mkdir -p $HOME/tmp $HOME/downloads $HOME/build

# https://apt.llvm.org/
#RUN bash -c "$(wget -O - https://apt.llvm.org/llvm.sh)"
RUN wget -q -O $HOME/downloads/llvm.sh https://apt.llvm.org/llvm.sh
RUN chmod +x $HOME/downloads/llvm.sh
RUN $HOME/downloads/llvm.sh 21

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
