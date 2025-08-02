FROM ubuntu:latest
LABEL maintainer="Jeremy Zheng"

ENV DEBIAN_FRONTEND noninteractive

RUN apt  update
RUN apt -y upgrade
RUN apt install -y lsb-release apt-utils \
    debian-keyring debian-archive-keyring apt-transport-https software-properties-common curl wget gnupg \
    sudo git locales locales-all \
    vim tzdata zip unzip tree tmux \
    build-essential cmake pkg-config libtool automake autoconf autoconf-archive binutils cpio \
    debhelper bison flex ninja-build \
    gawk \
    python3-full python3-dev \
    libncurses-dev \
    rsync bc bzr cvs mercurial subversion asciidoc w3m dblatex graphviz python3-matplotlib \
    texinfo help2man libtool-bin

RUN apt -y autoremove
RUN apt -y clean

RUN echo "en_US.UTF-8 UTF-8" > /etc/locale.gen
RUN locale-gen
RUN update-locale LANG=en_US.UTF-8
RUN update-alternatives --set editor /usr/bin/vim.basic

RUN useradd -s /bin/bash -m deploy
RUN passwd -l deploy
RUN echo 'deploy ALL=(ALL:ALL) NOPASSWD: ALL' > /etc/sudoers.d/101-deploy

USER deploy
RUN mkdir -p $HOME/downloads $HOME/build $HOME/local $HOME/tmp $HOME/src

RUN echo 'export LANG=en_US.UTF-8' >> $HOME/.bashrc
RUN echo 'export LC_ALL=en_US.UTF-8' >> $HOME/.bashrc
RUN echo 'export PATH=$HOME/.local/bin:$PATH' >> $HOME/.bashrc

RUN git config --global core.quotepath false \
    && git config --global http.version HTTP/1.1 \
    && git config --global pull.rebase false \
    && git config --global url."https://".insteadOf git://
RUN echo 'set-option -g history-limit 102400' > $HOME/.tmux.conf 

# https://github.com/crosstool-ng/crosstool-ng
# https://crosstool-ng.github.io/docs/
ENV CROSSTOOLNG_VERSION "crosstool-ng-1.27.0"
RUN git clone -b ${CROSSTOOLNG_VERSION} https://github.com/crosstool-ng/crosstool-ng.git $HOME/downloads/crosstool-ng
RUN cd $HOME/downloads/crosstool-ng \
    && ./bootstrap \
    && ./configure --enable-local \
    && make
ADD --chown=deploy crosstool-ng/toolchains/ /home/deploy/x-tools/

# ct-ng list-samples
# ct-ng arm-unknown-linux-gnueabi
# ct-ng menuconfig
# ct-ng build

# https://github.com/buildroot/buildroot
# https://buildroot.org/downloads/manual/manual.html#requirement
ENV BUILDROOT_VERSION "2025.02.4"
RUN git clone -b ${BUILDROOT_VERSION} https://github.com/buildroot/buildroot.git $HOME/downloads/buildroot
RUN echo 'export BR2_DL_DIR=$HOME/downloads' >> $HOME/.bashrc
# make list-defconfigs
RUN bash -c "source $HOME/.bashrc \
    && cd $HOME/downloads/buildroot \
    && make distclean \
    && make O=$HOME/build/rpi raspberrypi_defconfig \
    && cd $HOME/build/rpi \
    && make source"
RUN bash -c "source $HOME/.bashrc \
    && cd $HOME/downloads/buildroot \
    && make distclean \
    && make O=$HOME/build/rpi4 raspberrypi4_defconfig \
    && cd $HOME/build/rpi4 \
    && make source"
RUN bash -c "source $HOME/.bashrc \
    && cd $HOME/downloads/buildroot \
    && make distclean \
    && make O=$HOME/build/rpi4-64 raspberrypi4_64_defconfig \
    && cd $HOME/build/rpi4-64 \
    && make source"

# https://github.com/openwrt/openwrt
ENV OPENWRT_VERSION "v24.10.2"
RUN git clone -b ${OPENWRT_VERSION} https://github.com/openwrt/openwrt.git $HOME/downloads/openwrt
RUN bash -c "source $HOME/.bashrc \
    && cd $HOME/downloads/openwrt \
    && ./scripts/feeds update -a \
    && ./scripts/feeds install -a"

# https://learn.microsoft.com/en-us/vcpkg/get_started/get-started?pivots=shell-bash 
RUN git clone -b 2025.07.25 https://github.com/microsoft/vcpkg.git $HOME/local/vcpkg
RUN $HOME/local/vcpkg/bootstrap-vcpkg.sh
RUN echo 'export VCPKG_DISABLE_METRICS=1' >> $HOME/.bashrc
ADD vcpkg.patch /opt/
RUN cd $HOME/local/vcpkg && git apply /opt/vcpkg.patch
ADD --chown=deploy crosstool-ng/triplets/ /home/deploy/local/vcpkg/triplets/community/

RUN echo "$(date -u +%4Y%m%d%H%M%S)" | sudo tee /VERSION

VOLUME /workspace
WORKDIR /workspace

CMD ["/bin/bash", "-l"]
