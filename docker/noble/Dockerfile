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
RUN apt -y install zsh git locales locales-all \
    vim tzdata zip unzip tree tmux \
    build-essential g++ libstdc++-${CPP_STD_VERSION}-dev \
    crossbuild-essential-arm64 g++-aarch64-linux-gnu libstdc++-${CPP_STD_VERSION}-dev:arm64 \
    crossbuild-essential-armhf g++-arm-linux-gnueabihf libstdc++-${CPP_STD_VERSION}-dev:armhf \
    crossbuild-essential-riscv64 g++-riscv64-linux-gnu libstdc++-${CPP_STD_VERSION}-dev:riscv64 \
    clang clangd clang-tools clang-format clang-tidy lldb lld \
    cmake pkg-config libtool automake autoconf autoconf-archive binutils cpio mold \
    debhelper bison flex ninja-build \
    erlang elixir \
    python3-full python3-dev

RUN apt -y autoremove
RUN apt -y clean

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
RUN echo 'set-option -g history-limit 102400' > $HOME/.tmux.conf \
    && echo 'set-option -g default-shell "/bin/zsh"' >> $HOME/.tmux.conf

# https://musl.cc/
RUN wget -q -P $HOME/downloads https://more.musl.cc/x86_64-linux-musl/x86_64-linux-musl-cross.tgz \
    && cd /opt \
    && tar xf $HOME/downloads/x86_64-linux-musl-cross.tgz \
    && echo 'export PATH=/opt/x86_64-linux-musl-cross/bin:$PATH' >> $HOME/.zshrc
RUN wget -q -P $HOME/downloads https://more.musl.cc/x86_64-linux-musl/armv7l-linux-musleabihf-cross.tgz \
    && cd /opt \
    && tar xf $HOME/downloads/armv7l-linux-musleabihf-cross.tgz \
    && echo 'export PATH=/opt/armv7l-linux-musleabihf-cross/bin:$PATH' >> $HOME/.zshrc
RUN wget -q -P $HOME/downloads https://more.musl.cc/x86_64-linux-musl/aarch64-linux-musl-cross.tgz \
    && cd /opt \
    && tar xf $HOME/downloads/aarch64-linux-musl-cross.tgz \
    && echo 'export PATH=/opt/aarch64-linux-musl-cross/bin:$PATH' >> $HOME/.zshrc
RUN wget -q -P $HOME/downloads https://more.musl.cc/x86_64-linux-musl/riscv64-linux-musl-cross.tgz \
    && cd /opt \
    && tar xf $HOME/downloads/riscv64-linux-musl-cross.tgz \
    && echo 'export PATH=/opt/riscv64-linux-musl-cross/bin:$PATH' >> $HOME/.zshrc

# https://github.com/nvm-sh/nvm
ENV NVM_VERSION "v0.39.7"
RUN git clone -b ${NVM_VERSION} https://github.com/nvm-sh/nvm.git $HOME/.nvm
RUN echo 'export NVM_DIR="$HOME/.nvm"' >> $HOME/.zshrc
RUN echo '[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"' >> $HOME/.zshrc
RUN echo '[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"' >> $HOME/.zshrc
RUN zsh -c "source $HOME/.zshrc \
    && nvm install --lts"

ENV JDK_VERSION "22-open"
# https://docs.gradle.org/current/userguide/compatibility.html
RUN curl -s "https://get.sdkman.io" | bash
RUN sed -i -e 's/sdkman_auto_answer=false/sdkman_auto_answer=true/g' $HOME/.sdkman/etc/config
RUN zsh -c "source $HOME/.zshrc \
    && sdk install java ${JDK_VERSION} \
    && sdk install maven \
    && sdk install gradle"

# https://go.dev/doc/install
ENV GO_VERSION "1.22.5"
RUN wget -q -P $HOME/downloads https://go.dev/dl/go${GO_VERSION}.linux-amd64.tar.gz
RUN tar xf $HOME/downloads/go${GO_VERSION}.linux-amd64.tar.gz -C $HOME/local
RUN echo 'export PATH=$HOME/local/go/bin:$PATH' >> $HOME/.zshrc \
    && echo 'export GOPATH=$HOME/go' >> $HOME/.zshrc \
    && echo 'export PATH="$(go env GOPATH)/bin:$PATH"' >> $HOME/.zshrc

# https://grpc.io/docs/languages/go/quickstart/
RUN zsh -c "source $HOME/.zshrc \
    && go install google.golang.org/protobuf/cmd/protoc-gen-go@latest \
    && go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest"

# https://www.rust-lang.org/tools/install
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN zsh -c "source $HOME/.cargo/env \
    && rustup target add x86_64-unknown-linux-musl \
    && rustup target add armv7-unknown-linux-gnueabihf \
    && rustup target add aarch64-unknown-linux-gnu \
    && rustup target add aarch64-unknown-linux-musl \
    && rustup target add riscv64gc-unknown-linux-gnu"

# https://github.com/microsoft/vcpkg
RUN git clone https://github.com/microsoft/vcpkg.git $HOME/local/vcpkg
RUN $HOME/local/vcpkg/bootstrap-vcpkg.sh \
    && echo 'export VCPKG_DISABLE_METRICS=1' >> $HOME/.zshrc

# https://github.com/grpc/grpc
ENV GRPC_VERSION "v1.65.1"
RUN git clone --recurse-submodules -b $GRPC_VERSION https://github.com/grpc/grpc.git $HOME/downloads/grpc
RUN apt install -y libssl-dev
RUN zsh -c "source $HOME/.zshrc \
    && mkdir -pv $HOME/build/grpc \
    && cd $HOME/build/grpc \
    && cmake -DCMAKE_BUILD_TYPE=Release \
    -DgRPC_INSTALL=ON \
    -DgRPC_SSL_PROVIDER=package \
    -DgRPC_BUILD_TESTS=OFF \
    -DCMAKE_INSTALL_PREFIX=$HOME/.local $HOME/downloads/grpc \
    && make \
    && make install"

RUN echo "$(date -u +%4Y%m%d%H%M%S)" | tee /VERSION

VOLUME /workspace
WORKDIR /workspace

CMD ["/bin/zsh", "-l"]
