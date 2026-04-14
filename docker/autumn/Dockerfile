FROM ubuntu:latest
LABEL maintainer="Jeremy Zheng"

ENV DEBIAN_FRONTEND "noninteractive"

RUN apt update
RUN apt -y install lsb-release apt-utils debian-keyring debian-archive-keyring apt-transport-https software-properties-common curl wget gnupg \
    && apt -y clean
# https://launchpad.net/~ubuntu-toolchain-r/+archive/ubuntu/ppa
RUN add-apt-repository -y ppa:ubuntu-toolchain-r/ppa
# https://launchpad.net/~deadsnakes/+archive/ubuntu/ppa
RUN add-apt-repository -y ppa:deadsnakes/ppa
# https://launchpad.net/~ondrej/+archive/ubuntu/php
RUN add-apt-repository -y ppa:ondrej/php
RUN apt update && apt -y upgrade && apt -y clean

ENV PYTHON_VERSION "3.15"
RUN apt -y install pwgen git locales locales-all vim tzdata zip unzip tree tmux screen \
    curl wget rsync telnet iputils-ping lftp \
    openssh-client sshpass \
    python${PYTHON_VERSION}-full \
    && apt -y clean
ENV GCC_VERSION 14
RUN apt -y install cmake pkg-config libtool automake autoconf autoconf-archive cpio mold bison flex meson ninja-build debhelper \
    build-essential crossbuild-essential-amd64 crossbuild-essential-arm64 crossbuild-essential-armhf crossbuild-essential-riscv64 \
    g++-${GCC_VERSION} g++-${GCC_VERSION}-x86-64-linux-gnu g++-${GCC_VERSION}-aarch64-linux-gnu g++-${GCC_VERSION}-riscv64-linux-gnu \
    && apt -y clean
RUN apt install -y dialog asciidoc doxygen imagemagick ffmpeg graphviz pandoc texlive-full \
    && apt -y clean
RUN apt-get install -y fonts-dejavu-extra fonts-opensymbol fonts-lxgw-wenkai fonts-smiley-sans \
    fonts-noto-extra fonts-noto-cjk-extra fonts-noto-color-emoji \
    fonts-arphic-ukai fonts-arphic-uming  \
    fonts-wqy-microhei fonts-wqy-zenhei \
    fonts-cns11643-kai fonts-cns11643-sung \
    fonts-moe-standard-kai fonts-moe-standard-song \
    fonts-ipaexfont fonts-ipafont fonts-konatu fonts-ipafont-nonfree-jisx0208 fonts-ipafont-nonfree-uigothic \
    fonts-mikachan \
    fonts-tibetan-machine fonts-ddc-uchen fonts-monlam fonts-sambhota-tsugring fonts-sambhota-yigchung \
    && apt -y clean
RUN apt install -y postgresql redis rabbitmq-server vsftpd nginx \
    && apt -y clean
ENV PHP_VERSION "8.5"
# php${PHP_VERSION}-net-imap php${PHP_VERSION}-net-smtp 
RUN apt install -y php${PHP_VERSION}-fpm php${PHP_VERSION}-mbstring php${PHP_VERSION}-xml php${PHP_VERSION}-bcmath php${PHP_VERSION}-curl php${PHP_VERSION}-zip php${PHP_VERSION}-zmq php${PHP_VERSION}-imagick php${PHP_VERSION}-gd php${PHP_VERSION}-intl php${PHP_VERSION}-soap \
    php${PHP_VERSION}-mysql php${PHP_VERSION}-pgsql php${PHP_VERSION}-sqlite3 php${PHP_VERSION}-redis php${PHP_VERSION}-mongodb php${PHP_VERSION}-amqp \
    php${PHP_VERSION}-snmp \
    && apt -y clean
RUN apt install -y libpq-dev libmysqlclient-dev libsqlite3-dev libssl-dev libyaml-dev libffi-dev zlib1g-dev && apt -y clean

RUN mkdir -p $HOME/local $HOME/tmp $HOME/downloads $HOME/build

# https://apt.llvm.org/
ENV CLANG_VERSION "21"
RUN wget -q -O $HOME/downloads/llvm.sh https://apt.llvm.org/llvm.sh \
    && bash $HOME/downloads/llvm.sh ${CLANG_VERSION} \
    && rm $HOME/downloads/llvm.sh
RUN update-alternatives --install /usr/bin/gcc gcc /usr/bin/gcc-${GCC_VERSION} 100 \
    && update-alternatives --install /usr/bin/g++ g++ /usr/bin/g++-${GCC_VERSION} 100 \
    && update-alternatives --install /usr/bin/x86_64-linux-gnu-gcc x86_64-linux-gnu-gcc /usr/bin/x86_64-linux-gnu-gcc-${GCC_VERSION} 100 \
    && update-alternatives --install /usr/bin/x86_64-linux-gnu-g++ x86_64-linux-gnu-g++ /usr/bin/x86_64-linux-gnu-g++-${GCC_VERSION} 100 \
    && update-alternatives --install /usr/bin/aarch64-linux-gnu-gcc aarch64-linux-gnu-gcc /usr/bin/aarch64-linux-gnu-gcc-${GCC_VERSION} 100 \
    && update-alternatives --install /usr/bin/aarch64-linux-gnu-g++ aarch64-linux-gnu-g++ /usr/bin/aarch64-linux-gnu-g++-${GCC_VERSION} 100 \
    && update-alternatives --install /usr/bin/riscv64-linux-gnu-gcc riscv64-linux-gnu-gcc /usr/bin/riscv64-linux-gnu-gcc-${GCC_VERSION} 100 \
    && update-alternatives --install /usr/bin/riscv64-linux-gnu-g++ riscv64-linux-gnu-g++ /usr/bin/riscv64-linux-gnu-g++-${GCC_VERSION} 100 \
    && update-alternatives --install /usr/bin/clang clang /usr/bin/clang-${CLANG_VERSION} 100 \
    && update-alternatives --install /usr/bin/clang++ clang++ /usr/bin/clang++-${CLANG_VERSION} 100

RUN apt -y clean && apt -y autoremove

RUN echo "en_US.UTF-8 UTF-8" > /etc/locale.gen && locale-gen && update-locale LANG=en_US.UTF-8
RUN update-alternatives --set editor /usr/bin/vim.basic

RUN git config --global core.quotepath false \
    && git config --global http.version HTTP/1.1 \
    && git config --global pull.rebase false \
    && git config --global url."https://".insteadOf git://
RUN echo 'set-option -g history-limit 102400' > $HOME/.tmux.conf && echo 'set-option -g default-shell "/bin/bash"' >> $HOME/.tmux.conf

ARG TARGETARCH

# https://min.io/download#/linux
RUN wget -q -P $HOME/downloads/ https://dl.min.io/aistor/minio/release/linux-${TARGETARCH}/minio \
    && chmod +x $HOME/downloads/minio \
    && mv $HOME/downloads/minio /usr/local/bin/ \
    && wget -q -P $HOME/downloads/ https://dl.min.io/aistor/mc/release/linux-${TARGETARCH}/mc \
    && chmod +x $HOME/downloads/mc \
    && mv $HOME/downloads/mc /usr/local/bin/

# https://github.com/amacneil/dbmate
RUN curl -fsSL -o $HOME/downloads/dbmate https://github.com/amacneil/dbmate/releases/latest/download/dbmate-linux-${TARGETARCH} \
    && mv $HOME/downloads/dbmate /usr/local/bin/ \
    && chmod +x /usr/local/bin/dbmate

# https://getcomposer.org/doc/00-intro.md#installation-linux-unix-macos
RUN wget -q -O $HOME/downloads/composer-setup.php https://getcomposer.org/installer \
    && cd $HOME/downloads \
    && php${PHP_VERSION} $HOME/downloads/composer-setup.php --install-dir=/usr/local/bin --filename=composer${PHP_VERSION} \
    && rm $HOME/downloads/composer-setup.php

# https://github.com/nvm-sh/nvm
ENV NVM_VERSION "v0.40.4"
RUN git clone -b ${NVM_VERSION} https://github.com/nvm-sh/nvm.git $HOME/.nvm
RUN echo 'export NVM_DIR="$HOME/.nvm"' >> $HOME/.profile \
    && echo '[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"' >> $HOME/.profile \
    && echo '[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"' >> $HOME/.profile
RUN bash -c "source $HOME/.nvm/nvm.sh && nvm install --lts && npm install -g yarn"

# https://github.com/sdkman/sdkman-cli
# https://docs.gradle.org/current/userguide/compatibility.html
ENV JDK_VERSION "25.0.2-amzn"
RUN curl -s "https://get.sdkman.io" | bash
RUN sed -i -e 's/sdkman_auto_answer=false/sdkman_auto_answer=true/g' $HOME/.sdkman/etc/config
RUN bash -c "source $HOME/.sdkman/bin/sdkman-init.sh \
    && sdk install java ${JDK_VERSION} \
    && sdk default java ${JDK_VERSION} \
    && sdk install gradle \
    && sdk install maven"

# https://www.rust-lang.org/tools/install
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN bash -c "source $HOME/.cargo/env \
    && rustup target add armv7-unknown-linux-gnueabihf \
    && rustup target add aarch64-unknown-linux-gnu \
    && rustup target add x86_64-unknown-linux-musl \
    && rustup target add aarch64-unknown-linux-musl \
    && rustup target add loongarch64-unknown-linux-gnu"
RUN bash -c "source $HOME/.cargo/env && cargo install diesel_cli && cargo install mdbook"

# https://github.com/rbenv/rbenv
# https://github.com/rbenv/ruby-build/tree/master/share/ruby-build
ENV RUBY_VERSION "4.0.2"
RUN git clone https://github.com/rbenv/rbenv.git $HOME/.rbenv \
    && git clone https://github.com/rbenv/ruby-build.git $HOME/.rbenv/plugins/ruby-build \
    && git clone https://github.com/rbenv/rbenv-vars.git $HOME/.rbenv/plugins/rbenv-vars 
RUN echo 'eval "$(~/.rbenv/bin/rbenv init - --no-rehash bash)"' >> ~/.profile
# https://github.com/rbenv/ruby-build
RUN bash -c "source $HOME/.profile \
    && rbenv install ${RUBY_VERSION} \
    && rbenv global ${RUBY_VERSION} \
    && gem install bundler"

# https://go.dev/doc/install
ENV GO_VERSION "1.26.1"
RUN wget -q -P $HOME/downloads https://go.dev/dl/go${GO_VERSION}.linux-${TARGETARCH}.tar.gz
RUN tar xf $HOME/downloads/go${GO_VERSION}.linux-${TARGETARCH}.tar.gz -C $HOME/local \
    && rm $HOME/downloads/go${GO_VERSION}.linux-${TARGETARCH}.tar.gz
RUN echo 'export PATH=$HOME/local/go/bin:$PATH' >> $HOME/.profile \
    && echo 'export GOPATH=$HOME/go' >> $HOME/.profile \
    && echo 'export PATH=$(go env GOPATH)/bin:$PATH' >> $HOME/.profile
# https://gohugo.io/installation/linux/#build-from-source
RUN bash -c "source $HOME/.profile && CGO_ENABLED=1 go install -tags extended github.com/gohugoio/hugo@latest"

# https://xmake.io/guide/quick-start.html
RUN curl -fsSL https://xmake.io/shget.text | bash
RUN echo 'export XMAKE_ROOT=y' >> $HOME/.profile

# https://learn.microsoft.com/en-us/vcpkg/get_started/get-started?pivots=shell-bash
RUN echo 'export VCPKG_DISABLE_METRICS=1' >> $HOME/.profile

RUN python${PYTHON_VERSION} -m venv $HOME/local/python3
RUN bash -c "source $HOME/local/python3/bin/activate \
    && pip install cmake ninja"
RUN echo 'source $HOME/local/python3/bin/activate' >> $HOME/.profile

# https://opensearch.org/downloads.html#opensearch
# https://github.com/aparo/opensearch-analysis-ik?tab=readme-ov-file#install
# https://github.com/aparo/opensearch-analysis-stconvert?tab=readme-ov-file#installation
# https://github.com/aparo/opensearch-analysis-pinyin?tab=readme-ov-file#installation
ENV OPENSEARCH_VERSION "3.0.0"
RUN if [ "$TARGETARCH" = "amd64" ]; then \
        wget -q -O $HOME/downloads/opensearch-${OPENSEARCH_VERSION}-linux-${TARGETARCH}.tar.gz https://artifacts.opensearch.org/releases/bundle/opensearch/${OPENSEARCH_VERSION}/opensearch-${OPENSEARCH_VERSION}-linux-x64.tar.gz; \
    elif [ "$TARGETARCH" = "arm64" ]; then \
        wget -q -O $HOME/downloads/opensearch-${OPENSEARCH_VERSION}-linux-${TARGETARCH}.tar.gz https://artifacts.opensearch.org/releases/bundle/opensearch/${OPENSEARCH_VERSION}/opensearch-${OPENSEARCH_VERSION}-linux-arm64.tar.gz; \
    else \
        exit 1; \
    fi

RUN tar xf $HOME/downloads/opensearch-${OPENSEARCH_VERSION}-linux-${TARGETARCH}.tar.gz -C $HOME/tmp \
    && rm $HOME/downloads/opensearch-${OPENSEARCH_VERSION}-linux-${TARGETARCH}.tar.gz \
    && mv $HOME/tmp/opensearch-${OPENSEARCH_VERSION} /opt/opensearch \
    && cd /opt/opensearch/ \
    && bin/opensearch-plugin install https://github.com/aparo/opensearch-analysis-ik/releases/download/${OPENSEARCH_VERSION}/opensearch-analysis-ik.zip \
    && bin/opensearch-plugin install https://github.com/aparo/opensearch-analysis-stconvert/releases/download/${OPENSEARCH_VERSION}/opensearch-analysis-stconvert.zip \
    && bin/opensearch-plugin install https://github.com/aparo/opensearch-analysis-pinyin/releases/download/${OPENSEARCH_VERSION}/opensearch-analysis-pinyin.zip

# https://opensearch.org/docs/latest/opensearch/install/tar/
RUN cd /opt/opensearch/ \
    && echo "network.host: 0.0.0.0" >> config/opensearch.yml \
    && echo "discovery.type: single-node" >> config/opensearch.yml \
    && echo "plugins.security.disabled: true" >> config/opensearch.yml \
    && mkdir -p config/opensearch-performance-analyzer \
    && touch config/opensearch-performance-analyzer/plugin-stats-metadata \
    && cp -r config/opensearch-analysis-ik plugins/opensearch-analysis-ik/config

RUN mkdir -p /var/lib/minio/data && chown -R nobody:nogroup /var/lib/minio

# https://www.rabbitmq.com/access-control.html#loopback-users
RUN echo 'loopback_users = none' | tee -a /etc/rabbitmq/rabbitmq.conf \
    && echo 'NODE_IP_ADDRESS=0.0.0.0' >> /etc/rabbitmq/rabbitmq-env.conf \
    && rabbitmq-plugins enable rabbitmq_mqtt \
    && rabbitmq-plugins enable rabbitmq_stream \
    && rabbitmq-plugins enable rabbitmq_shovel rabbitmq_shovel_management

RUN mkdir -p /var/lib/postgres \
    && chown postgres:postgres /var/lib/postgres \
    && su -c "/usr/lib/postgresql/16/bin/initdb -D /var/lib/postgres/data" - postgres \
    && echo "listen_addresses = '0.0.0.0'" | tee -a /var/lib/postgres/data/postgresql.conf \
    && echo "host  all  all 0.0.0.0/0 md5" | tee -a /var/lib/postgres/data/pg_hba.conf

# php -i
# <?php php_info(); ?>
# <?php xdebug_info(); ?>
RUN sed -i 's/memory_limit =.*/memory_limit = 1G/' /etc/php/${PHP_VERSION}/fpm/php.ini \
    && sed -i 's/;extension=bz2/extension=bz2/' /etc/php/${PHP_VERSION}/fpm/php.ini \
    && sed -i 's/;extension=enchant/extension=enchant/' /etc/php/${PHP_VERSION}/fpm/php.ini \
    && sed -i 's/;extension=exif/extension=exif/' /etc/php/${PHP_VERSION}/fpm/php.ini \
    && sed -i 's/;extension=ftp/extension=ftp/' /etc/php/${PHP_VERSION}/fpm/php.ini \
    && sed -i 's/;extension=gd/extension=gd/' /etc/php/${PHP_VERSION}/fpm/php.ini \
    && sed -i 's/;extension=gettext/extension=gettext/' /etc/php/${PHP_VERSION}/fpm/php.ini \
    && sed -i 's/;extension=gmp/extension=gmp/' /etc/php/${PHP_VERSION}/fpm/php.ini \
    && sed -i 's/;extension=iconv/extension=iconv/' /etc/php/${PHP_VERSION}/fpm/php.ini \
    && sed -i 's/;extension=intl/extension=intl/' /etc/php/${PHP_VERSION}/fpm/php.ini \
    && sed -i 's/;extension=mysqli/extension=mysqli/' /etc/php/${PHP_VERSION}/fpm/php.ini \
    && sed -i 's/;extension=odbc/extension=odbc/' /etc/php/${PHP_VERSION}/fpm/php.ini \
    && sed -i 's/;extension=pdo_mysql/extension=pdo_mysql/' /etc/php/${PHP_VERSION}/fpm/php.ini \
    && sed -i 's/;extension=pdo_odbc/extension=pdo_odbc/' /etc/php/${PHP_VERSION}/fpm/php.ini \
    && sed -i 's/;extension=pdo_pgsql/extension=pdo_pgsql/' /etc/php/${PHP_VERSION}/fpm/php.ini \
    && sed -i 's/;extension=pdo_sqlite/extension=pdo_sqlite/' /etc/php/${PHP_VERSION}/fpm/php.ini \
    && sed -i 's/;extension=pgsql/extension=pgsql/' /etc/php/${PHP_VERSION}/fpm/php.ini \
    && sed -i 's/;extension=snmp/extension=snmp/' /etc/php/${PHP_VERSION}/fpm/php.ini \
    && sed -i 's/;extension=soap/extension=soap/' /etc/php/${PHP_VERSION}/fpm/php.ini \
    && sed -i 's/;extension=sockets/extension=sockets/' /etc/php/${PHP_VERSION}/fpm/php.ini \
    && sed -i 's/;extension=sodium/extension=sodium/' /etc/php/${PHP_VERSION}/fpm/php.ini \
    && sed -i 's/;extension=sqlite3/extension=sqlite3/' /etc/php/${PHP_VERSION}/fpm/php.ini \
    && sed -i 's/;extension=tidy/extension=tidy/' /etc/php/${PHP_VERSION}/fpm/php.ini \
    && sed -i 's/;extension=xsl/extension=xsl/' /etc/php/${PHP_VERSION}/fpm/php.ini
    

RUN echo "$(date -u +%4Y%m%d%H%M%S)" | tee /VERSION

EXPOSE 80/tcp 8080/tcp 9000/tcp 5432/tcp 6379/tcp 16379/tcp 5672/tcp 9200/tcp 9300/tcp 9600/tcp
VOLUME /srv
WORKDIR /srv

CMD ["/bin/bash", "-l"]
