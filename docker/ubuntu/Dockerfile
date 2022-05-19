FROM ubuntu:latest
LABEL maintainer="Jeremy Zheng"

ENV DEBIAN_FRONTEND noninteractive

RUN apt update
RUN apt -y upgrade
RUN apt -y install debian-keyring debian-archive-keyring apt-transport-https software-properties-common curl wget gnupg
RUN apt -y install zsh git locales locales-all rsync openssh-client sshpass \
    vim tzdata pwgen zip unzip tree tmux dialog \
    net-tools dnsutils net-tools iputils-arping iputils-ping telnet \
    imagemagick ffmpeg fonts-dejavu \
    build-essential cmake pkg-config libtool automake autoconf binutils cpio mold \
    debhelper bison flex ninja-build \
    python3 python3-distutils python3-dev python3-pip
RUN apt -y autoremove
RUN apt -y clean

RUN echo "en_US.UTF-8 UTF-8" > /etc/locale.gen
RUN locale-gen
RUN update-locale LANG=en_US.UTF-8
RUN update-alternatives --set editor /usr/bin/vim.basic

RUN mkdir -p $HOME/downloads $HOME/build $HOME/local

# https://github.com/ohmyzsh/ohmyzsh
RUN git clone https://github.com/ohmyzsh/ohmyzsh.git $HOME/.oh-my-zsh
RUN cp $HOME/.oh-my-zsh/templates/zshrc.zsh-template $HOME/.zshrc
RUN echo 'source $HOME/.profile' >> $HOME/.zshrc
RUN echo 'export LANG=en_US.UTF-8' >> $HOME/.profile
RUN echo 'export LC_ALL=en_US.UTF-8' >> $HOME/.profile

RUN git config --global core.quotepath false
RUN echo 'set-option -g history-limit 102400' > $HOME/.tmux.conf

RUN sh -c ". $HOME/.profile \
    && pip3 install --user ansible paramiko"
RUN echo 'export ANSIBLE_HOST_KEY_CHECKING=False' >> $HOME/.profile
RUN echo 'export PATH=$HOME/.local/bin:$PATH' >> $HOME/.profile
RUN echo 'alias peony="ANSIBLE_LOG_PATH=$HOME/$(date +%Y%m%d%H%M%S).log ansible-playbook"' >> $HOME/.profile

# https://github.com/nvm-sh/nvm
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.1/install.sh | sh
RUN sh -c ". $HOME/.profile \
    && nvm install node \
    && nvm use node \
    && npm i yarn -g"
RUN sh -c ". $HOME/.profile \
    && nvm install --lts \
    && nvm use --lts \
    && npm i yarn -g"
# https://stackoverflow.com/questions/37324519/node-sass-does-not-yet-support-your-current-environment-linux-64-bit-with-false
RUN sh -c ". $HOME/.profile \
    && nvm install lts/fermium \
    && nvm use lts/fermium \
    && npm i yarn -g"
RUN echo 'export PATH=$HOME/.yarn/bin:$PATH' >> $HOME/.profile


RUN curl -s "https://get.sdkman.io" | zsh
RUN sed -i -e 's/sdkman_auto_answer=false/sdkman_auto_answer=true/g' $HOME/.sdkman/etc/config
RUN zsh -c "source $HOME/.zshrc \
    && sdk install java 18.0.1.1-open \
    && sdk install maven \
    && sdk install gradle"

ENV CLANG_VERSION "14.0.0"
RUN wget -q -P ${HOME}/downloads/ \
    https://github.com/llvm/llvm-project/releases/download/llvmorg-${CLANG_VERSION}/clang+llvm-${CLANG_VERSION}-x86_64-linux-gnu-ubuntu-18.04.tar.xz
RUN tar xf ${HOME}/downloads/clang+llvm-${CLANG_VERSION}-x86_64-linux-gnu-ubuntu-18.04.tar.xz -C ${HOME}/local --no-same-owner

ENV GCC_ARM_VERSION "10.3-2021.07"
RUN wget -q -P ${HOME}/downloads/ \
    https://armkeil.blob.core.windows.net/developer/Files/downloads/gnu-a/${GCC_ARM_VERSION}/binrel/gcc-arm-${GCC_ARM_VERSION}-x86_64-aarch64-none-linux-gnu.tar.xz
RUN tar xf ${HOME}/downloads/gcc-arm-${GCC_ARM_VERSION}-x86_64-aarch64-none-linux-gnu.tar.xz -C ${HOME}/local --no-same-owner
RUN wget -q -P ${HOME}/downloads/ \
    https://armkeil.blob.core.windows.net/developer/Files/downloads/gnu-a/${GCC_ARM_VERSION}/binrel/gcc-arm-${GCC_ARM_VERSION}-x86_64-arm-none-linux-gnueabihf.tar.xz
RUN tar xf ${HOME}/downloads/gcc-arm-${GCC_ARM_VERSION}-x86_64-arm-none-linux-gnueabihf.tar.xz -C ${HOME}/local --no-same-owner

RUN echo "$(date -u +%4Y%m%d%H%M%S)" | tee /VERSION

VOLUME /workspace
WORKDIR /workspace

CMD ["/bin/zsh", "-l"]
