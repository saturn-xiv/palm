FROM ubuntu:latest
LABEL maintainer="Jeremy Zheng"

ENV DEBIAN_FRONTEND noninteractive

RUN apt update
RUN apt install -y lsb-release apt-utils \
    debian-keyring debian-archive-keyring apt-transport-https software-properties-common curl wget gnupg
RUN apt -y upgrade
RUN apt install -y git zip vim locales \
    pkg-config cmake flex bison ninja-build autoconf libtool mold \
    python3-full python3-dev libssl-dev libmariadb-dev libhiredis-dev
RUN apt -y autoremove
RUN apt -y clean

RUN echo "en_US.UTF-8 UTF-8" > /etc/locale.gen
RUN locale-gen

RUN echo "$(date -u +%4Y%m%d%H%M%S)" | sudo tee /VERSION

VOLUME /workspace
WORKDIR /workspace

CMD ["/bin/bash", "-l"]
