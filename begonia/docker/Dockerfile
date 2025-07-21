FROM ubuntu:latest
LABEL maintainer="Jeremy Zheng"

ENV DEBIAN_FRONTEND noninteractive

RUN apt update
RUN apt install -y lsb-release apt-utils \
    debian-keyring debian-archive-keyring apt-transport-https software-properties-common curl wget gnupg
ENV GCC_VERSION 14
# https://launchpad.net/~ubuntu-toolchain-r/+archive/ubuntu/test
RUN add-apt-repository -y ppa:ubuntu-toolchain-r/test
RUN apt -y upgrade
RUN apt -y install git locales locales-all vim tzdata zip unzip tree tmux \
    build-essential g++-${GCC_VERSION} \
    crossbuild-essential-arm64 g++-${GCC_VERSION}-aarch64-linux-gnu \
    crossbuild-essential-armhf g++-${GCC_VERSION}-arm-linux-gnueabihf \
    crossbuild-essential-riscv64 g++-${GCC_VERSION}-riscv64-linux-gnu \
    cmake pkg-config libtool automake autoconf autoconf-archive binutils cpio mold \
    bison flex ninja-build \
    python3-full python3-dev

# https://apt.llvm.org/
RUN bash -c "$(wget -O - https://apt.llvm.org/llvm.sh)"

RUN apt -y autoremove
RUN apt -y clean


RUN update-alternatives --install /usr/bin/cc cc /usr/bin/gcc-${GCC_VERSION} 100 
RUN update-alternatives --install /usr/bin/c++ c++ /usr/bin/g++-${GCC_VERSION} 100 
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
RUN update-locale LANG=en_US.UTF-8
RUN update-alternatives --set editor /usr/bin/vim.basic

RUN mkdir -p $HOME/downloads $HOME/build $HOME/local $HOME/tmp

# https://github.com/nvm-sh/nvm
ENV NVM_VERSION "v0.40.3"
RUN git clone -b ${NVM_VERSION} https://github.com/nvm-sh/nvm.git $HOME/.nvm
RUN echo 'export NVM_DIR="$HOME/.nvm"' >> $HOME/.bashrc
RUN echo '[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"' >> $HOME/.bashrc
RUN echo '[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"' >> $HOME/.bashrc
RUN bash -c ". $HOME/.nvm/nvm.sh && nvm install --lts"

RUN python3 -m venv $HOME/python3
RUN echo 'source $HOME/python3/bin/activate' >> $HOME/.bashrc
# https://pip.pypa.io/en/stable/installation/#get-pip-py
ADD https://bootstrap.pypa.io/get-pip.py /tmp/
RUN bash -c ". $HOME/python3/bin/activate && python3 /tmp/get-pip.py"
RUN bash -c ". $HOME/python3/bin/activate && pip install cmake==3.31.6"

# https://xmake.io/#/guide/installation?id=via-curl
RUN curl -fsSL https://xmake.io/shget.text | bash
RUN echo 'source $HOME/.xmake/profile' >> $HOME/.bashrc
RUN echo 'export XMAKE_ROOT=y' >> $HOME/.bashrc

# https://learn.microsoft.com/en-us/vcpkg/get_started/get-started?pivots=shell-bash 
RUN git clone -b 2025.06.13 https://github.com/microsoft/vcpkg.git $HOME/local/vcpkg
RUN $HOME/local/vcpkg/bootstrap-vcpkg.sh
RUN echo 'export VCPKG_ROOT=$HOME/local/vcpkg' >> $HOME/.bashrc
RUN echo 'export VCPKG_DISABLE_METRICS=1' >> $HOME/.bashrc
ADD vcpkg.patch /opt/
RUN cd $HOME/local/vcpkg && git apply /opt/vcpkg.patch

RUN git clone -b c++ https://github.com/saturn-xiv/palm.git $HOME/workspace/palm
# RUN cd $HOME/workspace/palm/ && ./scripts/build.sh

RUN echo "$(date -u +%4Y%m%d%H%M%S)" | tee /VERSION

VOLUME /workspace
WORKDIR /workspace

CMD ["/bin/bash", "-l"]
