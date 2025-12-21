FROM ubuntu:latest
LABEL maintainer="Jeremy Zheng"

ENV DEBIAN_FRONTEND="noninteractive"

RUN apt update
RUN apt -y upgrade
RUN apt install -y lsb-release apt-utils \
    debian-keyring debian-archive-keyring apt-transport-https software-properties-common curl wget gnupg

ENV GCC_VERSION=14
RUN apt -y install zsh git locales locales-all vim tzdata zip unzip tree screen \
    build-essential g++-${GCC_VERSION} \
    crossbuild-essential-arm64 g++-${GCC_VERSION}-aarch64-linux-gnu \
    crossbuild-essential-armhf g++-${GCC_VERSION}-arm-linux-gnueabihf \
    crossbuild-essential-riscv64 g++-${GCC_VERSION}-riscv64-linux-gnu \
    pkg-config libtool automake autoconf autoconf-archive binutils cpio bison flex ninja-build meson cmake mold \
    python3-full python3-dev
RUN apt -y autoremove
RUN apt -y clean

RUN update-alternatives --install /usr/bin/cc cc /usr/bin/gcc-${GCC_VERSION} 100 \
    && update-alternatives --install /usr/bin/c++ c++ /usr/bin/g++-${GCC_VERSION} 100 \
    && update-alternatives --install /usr/bin/gcc gcc /usr/bin/gcc-${GCC_VERSION} 100 \
    && update-alternatives --install /usr/bin/g++ g++ /usr/bin/g++-${GCC_VERSION} 100 \
    && update-alternatives --install /usr/bin/aarch64-linux-gnu-gcc aarch64-linux-gnu-gcc /usr/bin/aarch64-linux-gnu-gcc-${GCC_VERSION} 100 \
    && update-alternatives --install /usr/bin/aarch64-linux-gnu-g++ aarch64-linux-gnu-g++ /usr/bin/aarch64-linux-gnu-g++-${GCC_VERSION} 100 \
    && update-alternatives --install /usr/bin/arm-linux-gnueabihf-gcc arm-linux-gnueabihf-gcc /usr/bin/arm-linux-gnueabihf-gcc-${GCC_VERSION} 100 \
    && update-alternatives --install /usr/bin/arm-linux-gnueabihf-g++ arm-linux-gnueabihf-g++ /usr/bin/arm-linux-gnueabihf-g++-${GCC_VERSION} 100 \
    && update-alternatives --install /usr/bin/riscv64-linux-gnu-gcc riscv64-linux-gnu-gcc /usr/bin/riscv64-linux-gnu-gcc-${GCC_VERSION} 100 \
    && update-alternatives --install /usr/bin/riscv64-linux-gnu-g++ riscv64-linux-gnu-g++ /usr/bin/riscv64-linux-gnu-g++-${GCC_VERSION} 100

RUN echo "en_US.UTF-8 UTF-8" > /etc/locale.gen
RUN locale-gen
RUN update-locale LANG=en_US.UTF-8
RUN update-alternatives --set editor /usr/bin/vim.basic

RUN mkdir -p $HOME/downloads $HOME/build $HOME/local $HOME/tmp

# https://github.com/ohmyzsh/ohmyzsh
RUN git clone https://github.com/ohmyzsh/ohmyzsh.git $HOME/.oh-my-zsh
RUN cp $HOME/.oh-my-zsh/templates/zshrc.zsh-template $HOME/.zshrc
RUN echo 'export LANG=en_US.UTF-8' >> $HOME/.zshrc
RUN echo 'export LC_ALL=en_US.UTF-8' >> $HOME/.zshrc
RUN echo 'export PATH=$HOME/.local/bin:$PATH' >> $HOME/.zshrc

RUN git config --global core.quotepath false \
    && git config --global http.version HTTP/1.1 \
    && git config --global pull.rebase false \
    && git config --global url."https://".insteadOf git://

# https://github.com/cross-tools/musl-cross
RUN git clone https://github.com/cross-tools/musl-cross.git $HOME/build/musl-cross
ENV MUSL_TOOLCHAIN_VERSION=20250929
RUN wget -q -P $HOME/downloads https://github.com/cross-tools/musl-cross/releases/download/${MUSL_TOOLCHAIN_VERSION}/x86_64-unknown-linux-musl.tar.xz \
    && tar xf $HOME/downloads/x86_64-unknown-linux-musl.tar.xz -C $HOME/local \
    && echo 'export PATH=$HOME/local/x86_64-unknown-linux-musl/bin:$PATH' >> $HOME/.zshrc
RUN wget -q -P $HOME/downloads https://github.com/cross-tools/musl-cross/releases/download/${MUSL_TOOLCHAIN_VERSION}/aarch64-unknown-linux-musl.tar.xz \
    && tar xf $HOME/downloads/aarch64-unknown-linux-musl.tar.xz -C $HOME/local \
    && echo 'export PATH=$HOME/local/aarch64-unknown-linux-musl/bin:$PATH' >> $HOME/.zshrc
RUN wget -q -P $HOME/downloads https://github.com/cross-tools/musl-cross/releases/download/${MUSL_TOOLCHAIN_VERSION}/riscv64-unknown-linux-musl.tar.xz \
    && tar xf $HOME/downloads/riscv64-unknown-linux-musl.tar.xz -C $HOME/local \
    && echo 'export PATH=$HOME/local/x86_64-linux-musl-cross/bin:$PATH' >> $HOME/.zshrc
RUN wget -q -P $HOME/downloads https://github.com/cross-tools/musl-cross/releases/download/${MUSL_TOOLCHAIN_VERSION}/loongarch64-unknown-linux-musl.tar.xz \
    && tar xf $HOME/downloads/loongarch64-unknown-linux-musl.tar.xz -C $HOME/local \
    && echo 'export PATH=$HOME/local/loongarch64-unknown-linux-musl/bin:$PATH' >> $HOME/.zshrc

# https://releases.llvm.org/
ENV LLVM_VERSION="21.1.0"
RUN wget -q -P $HOME/downloads https://github.com/llvm/llvm-project/releases/download/llvmorg-${LLVM_VERSION}/LLVM-${LLVM_VERSION}-Linux-X64.tar.xz \
    && tar xf $HOME/downloads/LLVM-${LLVM_VERSION}-Linux-X64.tar.xz -C $HOME/local \
    && echo "export PATH=\$HOME/local/LLVM-${LLVM_VERSION}-Linux-X64/bin:\$PATH" >> $HOME/.zshrc

# https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads
ENV ARM_GNU_TOOLCHAIN_VERSION="14.3.rel1"
# RUN wget -q -P $HOME/downloads/ https://developer.arm.com/-/media/Files/downloads/gnu/${ARM_GNU_TOOLCHAIN_VERSION}/srcrel/arm-gnu-toolchain-src-snapshot-${ARM_GNU_TOOLCHAIN_VERSION}-manifest.txt
RUN wget -q -P $HOME/downloads https://developer.arm.com/-/media/Files/downloads/gnu/${ARM_GNU_TOOLCHAIN_VERSION}/binrel/arm-gnu-toolchain-${ARM_GNU_TOOLCHAIN_VERSION}-x86_64-arm-none-linux-gnueabihf.tar.xz \
    && tar xf $HOME/downloads/arm-gnu-toolchain-${ARM_GNU_TOOLCHAIN_VERSION}-x86_64-arm-none-linux-gnueabihf.tar.xz -C $HOME/local \
    && echo "export PATH=\$HOME/local/arm-gnu-toolchain-${ARM_GNU_TOOLCHAIN_VERSION}-x86_64-arm-none-linux-gnueabihf/bin:\$PATH" >> $HOME/.zshrc
RUN wget -q -P $HOME/downloads https://developer.arm.com/-/media/Files/downloads/gnu/${ARM_GNU_TOOLCHAIN_VERSION}/binrel/arm-gnu-toolchain-${ARM_GNU_TOOLCHAIN_VERSION}-x86_64-aarch64-none-linux-gnu.tar.xz \
    && tar xf $HOME/downloads/arm-gnu-toolchain-${ARM_GNU_TOOLCHAIN_VERSION}-x86_64-aarch64-none-linux-gnu.tar.xz -C $HOME/local \
    && echo "export PATH=\$HOME/local/arm-gnu-toolchain-${ARM_GNU_TOOLCHAIN_VERSION}-x86_64-aarch64-none-linux-gnu/bin:\$PATH" >> $HOME/.zshrc

# https://github.com/microsoft/vcpkg
RUN echo 'export VCPKG_DISABLE_METRICS=1' >> $HOME/.zshrc
RUN git clone https://github.com/microsoft/vcpkg.git $HOME/local/vcpkg


RUN echo "$(date -u +%4Y%m%d%H%M%S)" | tee /VERSION

VOLUME /workspace
WORKDIR /workspace

CMD ["/bin/zsh", "-l"]
