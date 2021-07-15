FROM ubuntu:focal
LABEL maintainer="Jeremy Zheng"

ENV DEBIAN_FRONTEND noninteractive

RUN apt update
RUN apt -y upgrade
RUN apt -y install apt-transport-https software-properties-common curl wget gnupg

# https://launchpad.net/~ubuntu-toolchain-r/+archive/ubuntu/test
RUN add-apt-repository -y ppa:ubuntu-toolchain-r/test -y
# https://launchpad.net/~deadsnakes/+archive/ubuntu/ppa
RUN add-apt-repository -y ppa:deadsnakes/ppa
# https://launchpad.net/~ondrej/+archive/ubuntu/php
RUN add-apt-repository -y ppa:ondrej/php
# https://www.postgresql.org/download/linux/ubuntu/
RUN echo "deb [arch=amd64] http://apt.postgresql.org/pub/repos/apt $(lsb_release -cs)-pgdg main" > /etc/apt/sources.list.d/pgdg.list
RUN wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | apt-key add -
# https://www.rabbitmq.com/install-debian.html
RUN apt-key adv --keyserver "hkps://keys.openpgp.org" --recv-keys "0x0A9AF2115F4687BD29803A206B73A36E6026DFCA"
RUN apt-key adv --keyserver "keyserver.ubuntu.com" --recv-keys "F77F1EDA57EBB1CC"
RUN apt-key adv --keyserver "keyserver.ubuntu.com" --recv-keys "F6609E60DC62814E"
RUN echo "deb [arch=amd64] http://ppa.launchpad.net/rabbitmq/rabbitmq-erlang/ubuntu $(lsb_release -cs) main" > /etc/apt/sources.list.d/rabbitmq.list
RUN echo "deb [arch=amd64] https://packagecloud.io/rabbitmq/rabbitmq-server/ubuntu/ $(lsb_release -cs) main" >> /etc/apt/sources.list.d/rabbitmq.list
# https://www.elastic.co/guide/en/elasticsearch/reference/current/deb.html
RUN wget -qO - https://artifacts.elastic.co/GPG-KEY-elasticsearch | apt-key add -
RUN echo "deb [arch=amd64] https://artifacts.elastic.co/packages/7.x/apt stable main" > /etc/apt/sources.list.d/elastic-7.x.list
# https://wiki.ubuntu.com/ToolChain
RUN dpkg --add-architecture armhf
RUN echo "deb [arch=amd64] http://archive.ubuntu.com/ubuntu/ $(lsb_release -cs) main restricted universe multiverse" > /etc/apt/sources.list
RUN echo "deb [arch=amd64] http://archive.ubuntu.com/ubuntu/ $(lsb_release -cs)-updates main restricted universe multiverse" >> /etc/apt/sources.list
RUN echo "deb [arch=amd64] http://archive.ubuntu.com/ubuntu/ $(lsb_release -cs)-security main restricted universe multiverse" >> /etc/apt/sources.list
RUN echo "deb [arch=armhf] http://ports.ubuntu.com/ $(lsb_release -cs) main restricted universe multiverse" >> /etc/apt/sources.list
RUN echo "deb [arch=armhf] http://ports.ubuntu.com/ $(lsb_release -cs)-security main restricted universe multiverse" >> /etc/apt/sources.list
RUN echo "deb [arch=armhf] http://ports.ubuntu.com/ $(lsb_release -cs)-updates main restricted universe multiverse" >> /etc/apt/sources.list
RUN apt update
RUN apt -y upgrade

RUN apt -y install zsh git locales rsync openssh-client sshpass \
    vim sudo tzdata pwgen curl zip unzip tree tmux screen \
    net-tools dnsutils net-tools iputils-arping iputils-ping telnet \
    imagemagick ffmpeg \
    build-essential pkg-config libtool automake autoconf binutils debhelper bison flex ninja-build \
    gcc-arm-linux-gnueabihf g++-arm-linux-gnueabihf gcc-multilib-arm-linux-gnueabihf g++-multilib-arm-linux-gnueabihf \
    pkg-config-arm-linux-gnueabihf binutils-arm-linux-gnueabihf \
    libgcc1-armhf-cross libsfgcc1-armhf-cross \
    libstdc++6-armhf-cross libstdc++-10-dev-armhf-cross \
    python3.9 python3.9-distutils python3.9-dev \
    php-cli php-fpm \
    php-xml php-json php-imap php-intl \
    php-mbstring php-bz2 php-zip php-curl \
    php-gd php-imagick \
    php-mysql php-pgsql php-sqlite3 php-redis \
    supervisor postgresql redis-server rabbitmq-server elasticsearch
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

RUN mkdir -p $HOME/downloads $HOME/local

# https://github.com/ohmyzsh/ohmyzsh
RUN git clone https://github.com/ohmyzsh/ohmyzsh.git $HOME/.oh-my-zsh
RUN cp $HOME/.oh-my-zsh/templates/zshrc.zsh-template $HOME/.zshrc
RUN echo 'source $HOME/.profile' >> $HOME/.zshrc

RUN echo 'term screen-256color' >> $HOME/.screenrc
RUN echo 'startup_message off' >> $HOME/.screenrc
RUN echo "defscrollback 10240" >> $HOME/.screenrc

RUN git config --global core.quotepath false

# https://pip.pypa.io/en/stable/installing/
RUN curl https://bootstrap.pypa.io/get-pip.py -o $HOME/downloads/get-pip.py
RUN python3.9 $HOME/downloads/get-pip.py --user
RUN sh -c ". $HOME/.profile \
    && pip install --user cmake"
RUN sh -c ". $HOME/.profile \
    && pip install --user ansible paramiko"
RUN echo 'export ANSIBLE_HOST_KEY_CHECKING=False' >> $HOME/.profile
RUN echo 'alias peony="ANSIBLE_LOG_PATH=$HOME/$(date +%Y%m%d%H%M%S).log ansible-playbook"' >> $HOME/.profile

# https://getcomposer.org/doc/00-intro.md#manual-installation
RUN wget https://getcomposer.org/download/latest-stable/composer.phar -O $HOME/.local/bin/composer
RUN chmod +x $HOME/.local/bin/composer

# https://github.com/nvm-sh/nvm
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.38.0/install.sh | sh
RUN sh -c ". $HOME/.profile \
    && nvm install node"
RUN sh -c ". $HOME/.profile \
    && npm i yarn -g"
RUN echo 'export PATH=$HOME/.yarn/bin:$PATH' >> $HOME/.profile

# https://sdkman.io/install
ENV JAVA_VERSION="16.0.1-open"
RUN curl -s "https://get.sdkman.io" | zsh
RUN sed -i -e 's/sdkman_auto_answer=false/sdkman_auto_answer=true/g' $HOME/.sdkman/etc/config
RUN zsh -c "source $HOME/.zshrc \
    && sdk install java ${JAVA_VERSION} \
    && sdk install maven \
    && sdk install gradle"

# https://golang.org/doc/install
ENV GO_VERSION "1.16.5"
RUN wget https://dl.google.com/go/go${GO_VERSION}.linux-amd64.tar.gz -O $HOME/downloads/go${GO_VERSION}.linux-amd64.tar.gz
RUN tar xf $HOME/downloads/go${GO_VERSION}.linux-amd64.tar.gz -C $HOME/local
RUN echo 'export PATH=$HOME/local/go/bin:$PATH' >> $HOME/.profile

# https://www.rust-lang.org/tools/install
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN zsh -c "source $HOME/.cargo/env \
    && rustup target add armv7-unknown-linux-gnueabihf"
RUN sudo apt install -y libpq-dev libmysqlclient-dev libsqlite3-dev
RUN zsh -c "source $HOME/.zshrc \
    && cargo install diesel_cli \
    && cargo install --locked cargo-outdated"

RUN sudo sed -i -e 's/daemonize yes/daemonize no/g' /etc/redis/redis.conf
COPY pg_hba.conf /etc/postgresql/13/main/
RUN sudo sed -i 's/^port = .*/port = 5432/' /etc/postgresql/13/main/postgresql.conf
COPY supervisor/ /etc/supervisor/conf.d/

RUN echo "$(date -u +%4Y%m%d%H%M%S)" | sudo tee -a /VERSION

VOLUME /workspace
WORKDIR /workspace

CMD ["/bin/zsh", "-l"]
