FROM alpine:latest
LABEL maintainer="Jeremy Zheng"

RUN apk update
RUN apk upgrade
RUN apk add perl vim curl zsh sudo pwgen openssl git tmux screen \
    build-base cmake \
    libudev-zero-dev sdl2-dev libunwind-dev \
    postgresql-dev mariadb-dev sqlite-dev

# deploy
RUN adduser -D -s /bin/zsh deploy
RUN echo 'deploy ALL=(ALL) NOPASSWD:ALL' > /etc/sudoers.d/101-deploy

USER deploy

RUN mkdir -p $HOME/downloads $HOME/local

# https://github.com/ohmyzsh/ohmyzsh
RUN git clone https://github.com/ohmyzsh/ohmyzsh.git $HOME/.oh-my-zsh
RUN cp $HOME/.oh-my-zsh/templates/zshrc.zsh-template $HOME/.zshrc
RUN echo 'source $HOME/.profile' >> $HOME/.zshrc

RUN echo 'term screen-256color' >> $HOME/.screenrc
RUN echo 'startup_message off' >> $HOME/.screenrc
RUN echo "defscrollback 10240" >> $HOME/.screenrc


# https://www.rust-lang.org/tools/install
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

VOLUME /workspace
WORKDIR /workspace

CMD ["/bin/zsh", "-l"]

