FROM ubuntu:focal
LABEL maintainer="Jeremy Zheng"

ENV DEBIAN_FRONTEND noninteractive

RUN apt update
RUN apt -y upgrade
RUN apt -y install debian-keyring debian-archive-keyring apt-transport-https software-properties-common curl wget gnupg

# https://launchpad.net/~ubuntu-toolchain-r/+archive/ubuntu/test
#RUN add-apt-repository -y ppa:ubuntu-toolchain-r/test -y

RUN apt -y install zsh git locales rsync openssh-client sshpass \
    vim sudo tzdata pwgen curl zip unzip tree tmux \
    net-tools dnsutils net-tools iputils-arping iputils-ping telnet \
    imagemagick ffmpeg \
    build-essential pkg-config libtool automake autoconf binutils cpio doxygen \
    debhelper bison flex ninja-build \
    python3 python3-distutils python3-pip python3-dev
RUN apt -y autoremove
RUN apt -y clean

RUN echo "en_US.UTF-8 UTF-8" > /etc/locale.gen
RUN locale-gen
RUN update-locale LANG=en_US.UTF-8
RUN update-alternatives --set editor /usr/bin/vim.basic

# deploy
RUN useradd -m deploy -s /bin/zsh
RUN passwd -l deploy
RUN echo 'deploy ALL=(ALL) NOPASSWD:ALL' > /etc/sudoers.d/101-deploy

USER deploy

RUN mkdir -p $HOME/downloads $HOME/local $HOME/build

# https://github.com/ohmyzsh/ohmyzsh
RUN git clone https://github.com/ohmyzsh/ohmyzsh.git $HOME/.oh-my-zsh
RUN cp $HOME/.oh-my-zsh/templates/zshrc.zsh-template $HOME/.zshrc
RUN echo 'source $HOME/.profile' >> $HOME/.zshrc
RUN echo 'export LANG=en_US.UTF-8' >> $HOME/.profile
RUN echo 'export LC_ALL=en_US.UTF-8' >> $HOME/.profile

RUN git config --global core.quotepath false
RUN echo 'set-option -g history-limit 10240' > $HOME/.tmux.conf

# https://pip.pypa.io/en/stable/installation/#get-pip-py
RUN curl -o- https://bootstrap.pypa.io/get-pip.py | python3
RUN sh -c ". $HOME/.profile \
    && pip install --user cmake"

RUN sh -c ". $HOME/.profile \
    && pip install --user ansible paramiko"
RUN echo 'export ANSIBLE_HOST_KEY_CHECKING=False' >> $HOME/.profile
RUN echo 'alias peony="ANSIBLE_LOG_PATH=$HOME/$(date +%Y%m%d%H%M%S).log ansible-playbook"' >> $HOME/.profile

# https://docs.conan.io/en/latest/getting_started.html#installing-dependencies
RUN sh -c ". $HOME/.profile \
    && pip install --user conan"

# https://github.com/nvm-sh/nvm
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | sh
RUN sh -c ". $HOME/.profile \
    && nvm install node"
RUN sh -c ". $HOME/.profile \
    && npm i yarn -g"
RUN echo 'export PATH=$HOME/.yarn/bin:$PATH' >> $HOME/.profile

# https://github.com/llvm/llvm-project/releases
ENV CLANG_VERSION "13.0.0"
RUN wget -P $HOME/downloads https://github.com/llvm/llvm-project/releases/download/llvmorg-${CLANG_VERSION}/clang+llvm-${CLANG_VERSION}-x86_64-linux-gnu-ubuntu-20.04.tar.xz
RUN tar xf $HOME/downloads/clang+llvm-${CLANG_VERSION}-x86_64-linux-gnu-ubuntu-20.04.tar.xz -C $HOME/local

# https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-a/downloads
ENV GNU_TOOLCHAIN_VERSION "10.3-2021.10"
RUN wget -P $HOME/downloads https://armkeil.blob.core.windows.net/developer/Files/downloads/gnu-a/${GNU_TOOLCHAIN_VERSION}/binrel/gcc-arm-${GNU_TOOLCHAIN_VERSION}-x86_64-arm-none-linux-gnueabihf.tar.xz
RUN tar xf $HOME/downloads/gcc-arm-${GNU_TOOLCHAIN_VERSION}-x86_64-arm-none-linux-gnueabihf.tar.xz -C $HOME/local
RUN wget -P $HOME/downloads https://armkeil.blob.core.windows.net/developer/Files/downloads/gnu-a/${GNU_TOOLCHAIN_VERSION}/binrel/gcc-arm-${GNU_TOOLCHAIN_VERSION}-x86_64-aarch64-none-linux-gnu.tar.xz
RUN tar xf $HOME/downloads/gcc-arm-${GNU_TOOLCHAIN_VERSION}-x86_64-aarch64-none-linux-gnu.tar.xz -C $HOME/local


# ADD grpc.sh /opt/
# RUN sudo apt install libssl-dev
# RUN zsh -c "source $HOME/.zshrc \
#     && /opt/grpc.sh 'v1.39.1' 'v3.17.1'"

RUN echo "$(date -u +%4Y%m%d%H%M%S)" | sudo tee /VERSION

VOLUME /workspace
WORKDIR /workspace

CMD ["/bin/zsh", "-l"]
