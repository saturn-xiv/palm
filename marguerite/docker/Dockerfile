FROM ubuntu:jammy
LABEL maintainer="Jeremy Zheng"

ENV DEBIAN_FRONTEND noninteractive

RUN apt update
RUN apt install -y lsb-release apt-utils \
    debian-keyring debian-archive-keyring apt-transport-https software-properties-common curl wget gnupg
RUN add-apt-repository -y ppa:rabbitmq/rabbitmq-erlang
RUN apt -y upgrade
RUN apt install -y git zip \
    build-essential pkg-config g++-12 cmake flex bison ninja-build autoconf libtool \
    linux-libc-dev \
    erlang elixir
# RUN update-alternatives --install /usr/bin/g++ g++ /usr/bin/g++-12 100
# RUN update-alternatives --install /usr/bin/gcc gcc /usr/bin/gcc-12 100

# libssl-dev zlib1g-dev

RUN echo "$(date -u +%4Y%m%d%H%M%S)" | tee /VERSION

VOLUME /workspace
WORKDIR /workspace

CMD ["/bin/bash", "-l"]
