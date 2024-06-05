FROM ubuntu:noble
LABEL maintainer="Jeremy Zheng"

ENV DEBIAN_FRONTEND noninteractive

RUN apt update
RUN apt install -y lsb-release apt-utils \
    debian-keyring debian-archive-keyring apt-transport-https software-properties-common curl wget gnupg
# https://wiki.debian.org/ToolChain/Cross
RUN dpkg --add-architecture armhf
RUN dpkg --add-architecture arm64
RUN dpkg --add-architecture riscv64
RUN perl -pi -e '$_ .= qq(Architectures: amd64\n) if /Components: main/' /etc/apt/sources.list.d/ubuntu.sources
RUN echo "deb [arch=armhf,arm64,riscv64] http://ports.ubuntu.com/ $(lsb_release -cs) main restricted universe multiverse" > /etc/apt/sources.list.d/ports.list
RUN echo "deb [arch=armhf,arm64,riscv64] http://ports.ubuntu.com/ $(lsb_release -cs)-security main restricted universe multiverse" >> /etc/apt/sources.list.d/ports.list
RUN echo "deb [arch=armhf,arm64,riscv64] http://ports.ubuntu.com/ $(lsb_release -cs)-updates main restricted universe multiverse" >> /etc/apt/sources.list.d/ports.list
RUN apt update
RUN apt -y upgrade
ENV CPP_STD_VERSION 13
RUN apt install -y vim sudo locales locales-all tree zsh \
    build-essential xmake ccache 7zip git file \
    clang clangd clang-tools clang-format clang-tidy lldb lld \
    crossbuild-essential-arm64 g++-aarch64-linux-gnu libstdc++-${CPP_STD_VERSION}-dev:arm64 \
    crossbuild-essential-armhf g++-arm-linux-gnueabihf libstdc++-${CPP_STD_VERSION}-dev:armhf \
    crossbuild-essential-riscv64 g++-riscv64-linux-gnu libstdc++-${CPP_STD_VERSION}-dev:riscv64 \
    cmake pkg-config libtool automake autoconf autoconf-archive binutils cpio mold \
    debhelper bison flex ninja-build \
    python3-full python3-dev
RUN apt -y autoremove
RUN apt -y clean

RUN echo "en_US.UTF-8 UTF-8" > /etc/locale.gen
RUN locale-gen
RUN update-locale LANG=en_US.UTF-8
RUN update-alternatives --set editor /usr/bin/vim.basic

RUN useradd -s /bin/zsh -m deploy
RUN passwd -l deploy
RUN echo 'deploy ALL=(ALL:ALL) NOPASSWD: ALL' > /etc/sudoers.d/101-deploy
USER deploy
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
RUN echo 'set-option -g history-limit 102400' > $HOME/.tmux.conf \
    && echo 'set-option -g default-shell "/bin/zsh"' >> $HOME/.tmux.conf

# https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads
RUN wget -q -P $HOME/downloads https://developer.arm.com/-/media/Files/downloads/gnu/13.2.rel1/binrel/arm-gnu-toolchain-13.2.rel1-x86_64-aarch64-none-linux-gnu.tar.xz \
    && tar xf $HOME/downloads/arm-gnu-toolchain-13.2.rel1-x86_64-aarch64-none-linux-gnu.tar.xz -C $HOME/local \
    && echo 'export PATH=$HOME/local/arm-gnu-toolchain-13.2.Rel1-x86_64-aarch64-none-linux-gnu/bin:$PATH' >> $HOME/.zshrc

# https://musl.cc/
RUN wget -q -P $HOME/downloads https://more.musl.cc/x86_64-linux-musl/x86_64-linux-musl-cross.tgz \
    && tar xf $HOME/downloads/x86_64-linux-musl-cross.tgz -C $HOME/local \
    && echo 'export PATH=$HOME/local/x86_64-linux-musl-cross/bin:$PATH' >> $HOME/.zshrc
RUN wget -q -P $HOME/downloads https://more.musl.cc/x86_64-linux-musl/armv7l-linux-musleabihf-cross.tgz \
    && tar xf $HOME/downloads/armv7l-linux-musleabihf-cross.tgz -C $HOME/local \
    && echo 'export PATH=$HOME/local/armv7l-linux-musleabihf-cross/bin:$PATH' >> $HOME/.zshrc
RUN wget -q -P $HOME/downloads https://more.musl.cc/x86_64-linux-musl/aarch64-linux-musl-cross.tgz \
    && tar xf $HOME/downloads/aarch64-linux-musl-cross.tgz -C $HOME/local \
    && echo 'export PATH=$HOME/local/aarch64-linux-musl-cross/bin:$PATH' >> $HOME/.zshrc
RUN wget -q -P $HOME/downloads https://more.musl.cc/x86_64-linux-musl/riscv64-linux-musl-cross.tgz \
    && tar xf $HOME/downloads/riscv64-linux-musl-cross.tgz -C $HOME/local \
    && echo 'export PATH=$HOME/local/riscv64-linux-musl-cross/bin:$PATH' >> $HOME/.zshrc


# https://github.com/nvm-sh/nvm
ENV NVM_VERSION "v0.39.7"
RUN git clone -b ${NVM_VERSION} https://github.com/nvm-sh/nvm.git $HOME/.nvm
RUN echo 'export NVM_DIR="$HOME/.nvm"' >> $HOME/.zshrc
RUN echo '[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"' >> $HOME/.zshrc
RUN echo '[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"' >> $HOME/.zshrc
RUN zsh -c "source $HOME/.zshrc \
    && nvm install --lts"

RUN echo "$(date -u +%4Y%m%d%H%M%S)" | sudo tee /VERSION

VOLUME /workspace
WORKDIR /workspace

CMD ["/bin/zsh", "-l"]
