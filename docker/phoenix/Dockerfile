FROM ubuntu:latest
LABEL maintainer="Jeremy Zheng"

ENV DEBIAN_FRONTEND noninteractive

RUN apt update
RUN apt -y install build-essential autoconf m4 libncurses5-dev libwxgtk3.2-dev libwxgtk-webview3.2-dev libgl1-mesa-dev libglu1-mesa-dev libpng-dev libssh-dev unixodbc-dev xsltproc fop libxml2-utils libncurses-dev openjdk-11-jdk \
    curl wget git zip \
    lsb-release zsh vim tmux locales locales-all 
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

# https://asdf-vm.com/guide/getting-started.html
# https://github.com/asdf-vm/asdf-erlang
# https://github.com/asdf-vm/asdf-elixir
RUN git clone https://github.com/asdf-vm/asdf.git $HOME/.asdf --branch v0.14.0
RUN echo 'source $HOME/.asdf/asdf.sh' >> $HOME/.zshrc 
RUN echo 'export KERL_CONFIGURE_OPTIONS="--disable-debug --without-javac"' >> $HOME/.zshrc 
RUN zsh -c "source $HOME/.zshrc \
    && asdf plugin add erlang https://github.com/asdf-vm/asdf-erlang.git \
    && asdf plugin-add elixir https://github.com/asdf-vm/asdf-elixir.git \
    && asdf install erlang latest \
    && asdf global erlang latest \
    && asdf install elixir latest \
    && asdf global elixir latest"

# https://go.dev/doc/install
ENV GO_VERSION "1.22.3"
RUN wget -q -P $HOME/downloads https://go.dev/dl/go${GO_VERSION}.linux-amd64.tar.gz
RUN tar xf $HOME/downloads/go${GO_VERSION}.linux-amd64.tar.gz -C $HOME/local
RUN echo 'export PATH=$HOME/local/go/bin:$PATH' >> $HOME/.zshrc \
    && echo 'export GOPATH=$HOME/go' >> $HOME/.zshrc \
    && echo 'export PATH="$(go env GOPATH)/bin:$PATH"' >> $HOME/.zshrc

RUN echo "$(date -u +%4Y%m%d%H%M%S)" | tee /VERSION

VOLUME /workspace
WORKDIR /workspace

CMD ["/bin/zsh", "-l"]
