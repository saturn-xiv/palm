FROM ubuntu:focal
LABEL maintainer="Jeremy Zheng"

ENV DEBIAN_FRONTEND noninteractive

RUN apt update
RUN apt -y upgrade
RUN apt -y install apt-transport-https software-properties-common curl wget gnupg

# https://launchpad.net/~ubuntu-toolchain-r/+archive/ubuntu/test
RUN add-apt-repository -y ppa:ubuntu-toolchain-r/test -y
# https://launchpad.net/~deadsnakes/+archive/ubuntu/ppa
# RUN add-apt-repository -y ppa:deadsnakes/ppa
# https://launchpad.net/~ondrej/+archive/ubuntu/php
RUN add-apt-repository -y ppa:ondrej/php
# https://www.nginx.com/resources/wiki/start/topics/tutorials/install/
RUN add-apt-repository -y ppa:nginx/stable
# https://www.postgresql.org/download/linux/ubuntu/
RUN echo "deb [arch=amd64] http://apt.postgresql.org/pub/repos/apt $(lsb_release -cs)-pgdg main" > /etc/apt/sources.list.d/pgdg.list
RUN wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | apt-key add -
# https://www.erlang-solutions.com/downloads/
RUN echo "deb [arch=amd64] https://packages.erlang-solutions.com/ubuntu $(lsb_release -cs) contrib" > /etc/apt/sources.list.d/erlang.list
RUN wget --quiet -O - https://packages.erlang-solutions.com/ubuntu/erlang_solutions.asc | apt-key add -
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
    python3 python3-distutils python3-pip python3-dev \
    php-cli php-fpm \
    php-xml php-json php-imap php-intl \
    php-mbstring php-bz2 php-zip php-curl \
    php-gd php-imagick \
    php-mysql php-pgsql php-sqlite3 php-redis \
    erlang \
    nginx mariadb-server postgresql redis-server
RUN apt -y autoremove
RUN apt -y clean

RUN echo "en_US.UTF-8 UTF-8" > /etc/locale.gen
RUN locale-gen
RUN update-locale LANG=en_US.UTF-8
RUN update-alternatives --set editor /usr/bin/vim.basic

RUN pip3 install supervisor

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
RUN python3 $HOME/downloads/get-pip.py --user
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
    && nvm install node --lts"
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


# https://min.io/download#/linux
RUN wget https://dl.min.io/server/minio/release/linux-amd64/minio -O $HOME/.local/bin/minio
RUN chmod +x $HOME/.local/bin/minio
RUN sudo mkdir -p /var/lib/minio/data
RUN sudo chown -R deploy:deploy /var/lib/minio/data

# https://www.elastic.co/guide/en/elasticsearch/reference/current/targz.html#install-linux
ENV ELASTICSEARCH_VERSION "7.13.4"
RUN wget https://artifacts.elastic.co/downloads/elasticsearch/elasticsearch-${ELASTICSEARCH_VERSION}-linux-x86_64.tar.gz -O $HOME/downloads/elasticsearch-${ELASTICSEARCH_VERSION}-linux-x86_64.tar.gz
RUN tar xf $HOME/downloads/elasticsearch-${ELASTICSEARCH_VERSION}-linux-x86_64.tar.gz -C $HOME/local
RUN ln -s $HOME/local/elasticsearch-${ELASTICSEARCH_VERSION} $HOME/local/elasticsearch

# https://www.rabbitmq.com/install-generic-unix.html
ENV RABBITMQ_VERSION "3.8.19"
RUN wget https://github.com/rabbitmq/rabbitmq-server/releases/download/v${RABBITMQ_VERSION}/rabbitmq-server-generic-unix-${RABBITMQ_VERSION}.tar.xz -O $HOME/downloads/rabbitmq-server-generic-unix-${RABBITMQ_VERSION}.tar.xz
RUN tar xf $HOME/downloads/rabbitmq-server-generic-unix-${RABBITMQ_VERSION}.tar.xz -C $HOME/local
RUN ln -s $HOME/local/rabbitmq_server-${RABBITMQ_VERSION} $HOME/local/rabbitmq_server

# setup nginx
COPY nginx.conf /etc/nginx/sites-available/default
RUN echo "daemon off;" | sudo tee /etc/nginx/modules-enabled/100-disable-daemon.conf
RUN sudo mkdir -p /var/lib/nginx
RUN sudo touch /run/nginx.pid
RUN sudo chown -R www-data:www-data /var/lib/nginx /run/nginx.pid

# setup php-fpm
RUN sudo mkdir -p /run/php

# setup redis
# RUN sudo sed -i -e 's/daemonize yes/daemonize no/g' /etc/redis/redis.conf
COPY redis/* /etc/redis/
RUN sudo mkdir -p /var/lib/redis/server
RUN for i in $(seq 1 6); do sudo mkdir -p /var/lib/redis/cluster-$i; done
RUN echo "vm.overcommit_memory = 1" | sudo tee /etc/sysctl.d/90-redis.conf

# setup postgresql
COPY pg_hba.conf /etc/postgresql/13/main/
RUN sudo sed -i 's/^port = .*/port = 5432/' /etc/postgresql/13/main/postgresql.conf

# setup mysql
RUN sudo mkdir /run/mysqld
RUN sudo chown -R root:mysql /run/mysqld
RUN sudo chmod 775 /run/mysqld

# setup rabbitmq
RUN sudo mkdir -p /var/lib/rabbitmq
RUN sudo chown -R deploy:deploy /var/lib/rabbitmq
RUN sudo touch /var/lib/rabbitmq/.erlang.cookie

# setup supervisord
COPY supervisor/ /etc/supervisor
RUN sudo mkdir -p /var/log/supervisor


# set open files
# ulimit -a; ulimit -Sn; ulimit -Hn
# COPY limits.d/* /etc/security/limits.d/
# RUN echo "session required pam_limits.so" | sudo tee /etc/pam.d/common-session
# RUN echo "fs.file-max = 2048000" | sudo tee /etc/sysctl.d/91-fs.conf


RUN echo "$(date -u +%4Y%m%d%H%M%S)" | sudo tee /VERSION

VOLUME /workspace
WORKDIR /workspace

CMD ["/bin/zsh", "-l"]
