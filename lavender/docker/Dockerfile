FROM ubuntu:latest
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
RUN apt update
RUN apt -y upgrade
ENV GCC_VERSION 13
RUN apt install -y git zip sudo vim \
    build-essential crossbuild-essential-arm64 crossbuild-essential-armhf crossbuild-essential-riscv64 \
    pkg-config cmake flex bison ninja-build autoconf libtool mold \
    g++-${GCC_VERSION} g++-${GCC_VERSION}-aarch64-linux-gnu gcc-${GCC_VERSION}-arm-linux-gnueabihf g++-${GCC_VERSION}-riscv64-linux-gnu \
    clang-${CLANG_VERSION} clangd-${CLANG_VERSION} clang-tools-${CLANG_VERSION} clang-format-${CLANG_VERSION} clang-tidy-${CLANG_VERSION} lldb-${CLANG_VERSION} lld-${CLANG_VERSION} \
    linux-libc-dev

RUN update-alternatives --install /usr/bin/clang++ clang++ /usr/bin/clang++-${CLANG_VERSION} 100
RUN update-alternatives --install /usr/bin/clang clang /usr/bin/clang-${CLANG_VERSION} 100
RUN update-alternatives --install /usr/bin/clang-format clang-format /usr/bin/clang-format-${CLANG_VERSION} 100
RUN update-alternatives --install /usr/bin/clang-tidy clang-tidy /usr/bin/clang-tidy-${CLANG_VERSION} 100
RUN update-alternatives --install /usr/bin/lldb lldb /usr/bin/lldb-${CLANG_VERSION} 100
RUN update-alternatives --install /usr/bin/lld lld /usr/bin/lld-${CLANG_VERSION} 100
RUN update-alternatives --install /usr/bin/gcc gcc /usr/bin/gcc-${GCC_VERSION} 100
RUN update-alternatives --install /usr/bin/g++ g++ /usr/bin/g++-${GCC_VERSION} 100
RUN update-alternatives --install /usr/bin/aarch64-linux-gnu-gcc aarch64-linux-gnu-gcc /usr/bin/aarch64-linux-gnu-gcc-${GCC_VERSION} 100
RUN update-alternatives --install /usr/bin/aarch64-linux-gnu-g++ aarch64-linux-gnu-g++ /usr/bin/aarch64-linux-gnu-g++-${GCC_VERSION} 100
RUN update-alternatives --install /usr/bin/arm-linux-gnueabihf-gcc arm-linux-gnueabihf-gcc /usr/bin/arm-linux-gnueabihf-gcc-${GCC_VERSION} 100
RUN update-alternatives --install /usr/bin/arm-linux-gnueabihf-g++ arm-linux-gnueabihf-g++ /usr/bin/arm-linux-gnueabihf-g++-${GCC_VERSION} 100
RUN update-alternatives --install /usr/bin/riscv64-linux-gnu-gcc riscv64-linux-gnu-gcc /usr/bin/riscv64-linux-gnu-gcc-${GCC_VERSION} 100
RUN update-alternatives --install /usr/bin/riscv64-linux-gnu-g++ riscv64-linux-gnu-g++ /usr/bin/riscv64-linux-gnu-g++-${GCC_VERSION} 100

RUN echo "en_US.UTF-8 UTF-8" > /etc/locale.gen
RUN locale-gen

RUN useradd -s /bin/bash -m deploy
RUN passwd -l deploy
RUN echo 'deploy ALL=(ALL:ALL) NOPASSWD: ALL' > /etc/sudoers.d/101-deploy

USER deploy
# https://xmake.io/#/guide/installation
RUN wget https://xmake.io/shget.text -O - | bash

RUN echo "$(date -u +%4Y%m%d%H%M%S)" | tee /VERSION

VOLUME /workspace
WORKDIR /workspace

CMD ["/bin/bash", "-l"]
