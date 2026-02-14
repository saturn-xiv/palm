FROM ubuntu:latest
LABEL maintainer="Jeremy Zheng"

ENV DEBIAN_FRONTEND=noninteractive

RUN apt update
RUN apt -y upgrade && apt -y clean && apt -y autoremove

RUN apt -y install curl wget rsync pwgen git locales locales-all vim tzdata zip unzip tree tmux screen openssh-client sshpass \
    python3-full \
    cmake pkg-config libtool automake autoconf autoconf-archive cpio mold bison flex ninja-build debhelper \
    clang clangd clang-tools clang-format clang-tidy lldb lld \
    build-essential crossbuild-essential-arm64 crossbuild-essential-armhf crossbuild-essential-riscv64 \
    && apt -y clean
RUN apt install -y  dialog asciidoc doxygen imagemagick ffmpeg graphviz pandoc texlive-full \
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
RUN apt install -y postgresql redis rabbitmq-server nginx \
    && apt -y clean
RUN apt install -y php-fpm php-mbstring php-xml php-bcmath php-curl php-zip php-zmq php-imagick php-gd php-intl php-soap \
    php-mysql php-pgsql php-sqlite3 php-redis php-mongodb php-amqp \
    php-net-imap php-net-smtp \
    && apt -y clean

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
RUN wget -q -O $HOME/downloads/composer https://getcomposer.org/installer \
    && cd $HOME/downloads \
    && php composer \
    && mv composer.phar /usr/local/bin/composer

# https://github.com/nvm-sh/nvm
ENV NVM_VERSION="v0.40.4"
RUN git clone -b ${NVM_VERSION} https://github.com/nvm-sh/nvm.git $HOME/.nvm
RUN echo 'export NVM_DIR="$HOME/.nvm"' >> $HOME/.profile \
    && echo '[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"' >> $HOME/.profile \
    && echo '[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"' >> $HOME/.profile
RUN bash -c "source $HOME/.nvm/nvm.sh && nvm install --lts && nvm use --lts"

# https://github.com/sdkman/sdkman-cli
# https://docs.gradle.org/current/userguide/compatibility.html
ENV JDK_VERSION="25.0.2-amzn"
RUN curl -s "https://get.sdkman.io" | bash
RUN sed -i -e 's/sdkman_auto_answer=false/sdkman_auto_answer=true/g' $HOME/.sdkman/etc/config
RUN bash -c "source $HOME/.sdkman/bin/sdkman-init.sh && sdk install java ${JDK_VERSION} && sdk default java ${JDK_VERSION}"

# https://opensearch.org/downloads.html#opensearch
# https://github.com/aparo/opensearch-analysis-ik?tab=readme-ov-file#install
# https://github.com/aparo/opensearch-analysis-stconvert?tab=readme-ov-file#installation
# https://github.com/aparo/opensearch-analysis-pinyin?tab=readme-ov-file#installation
ENV OPENSEARCH_VERSION="3.0.0"
RUN if [ "$TARGETARCH" = "amd64" ]; then \
        wget -q -O $HOME/downloads/opensearch-${OPENSEARCH_VERSION}-linux-${TARGETARCH}.tar.gz https://artifacts.opensearch.org/releases/bundle/opensearch/${OPENSEARCH_VERSION}/opensearch-${OPENSEARCH_VERSION}-linux-x64.tar.gz; \
    elif [ "$TARGETARCH" = "arm64" ]; then \
        wget -q -O $HOME/downloads/opensearch-${OPENSEARCH_VERSION}-linux-${TARGETARCH}.tar.gz https://artifacts.opensearch.org/releases/bundle/opensearch/${OPENSEARCH_VERSION}/opensearch-${OPENSEARCH_VERSION}-linux-arm64.tar.gz; \
    else \
        exit 1; \
    fi

RUN mkdir -p $HOME/tmp
RUN tar xf $HOME/downloads/opensearch-${OPENSEARCH_VERSION}-linux-${TARGETARCH}.tar.gz -C $HOME/tmp \
    && rm $HOME/downloads/opensearch-${OPENSEARCH_VERSION}-linux-${TARGETARCH}.tar.gz \
    && mv $HOME/tmp/opensearch-${OPENSEARCH_VERSION} /opt/opensearch \
    && cd /opt/opensearch/ \
    && bin/opensearch-plugin install https://github.com/aparo/opensearch-analysis-ik/releases/download/${OPENSEARCH_VERSION}/opensearch-analysis-ik.zip \
    && bin/opensearch-plugin install https://github.com/aparo/opensearch-analysis-stconvert/releases/download/${OPENSEARCH_VERSION}/opensearch-analysis-stconvert.zip \
    && bin/opensearch-plugin install https://github.com/aparo/opensearch-analysis-pinyin/releases/download/${OPENSEARCH_VERSION}/opensearch-analysis-pinyin.zip

# https://xmake.io/guide/quick-start.html
RUN curl -fsSL https://xmake.io/shget.text | bash
RUN echo 'export XMAKE_ROOT=y' >> $HOME/.profile

# https://learn.microsoft.com/en-us/vcpkg/get_started/get-started?pivots=shell-bash
RUN echo 'export VCPKG_DISABLE_METRICS=1' >> $HOME/.profile

# https://opensearch.org/docs/latest/opensearch/install/tar/
RUN echo "network.host: 0.0.0.0" >> /opt/opensearch/config/opensearch.yml \
    && echo "discovery.type: single-node" >> /opt/opensearch/config/opensearch.yml \
    && echo "plugins.security.disabled: true" >> /opt/opensearch/config/opensearch.yml
# https://github.com/infinilabs/analysis-stconvert
RUN cd /opt/opensearch/ && bin/opensearch-plugin install https://get.infini.cloud/opensearch/analysis-stconvert/${OPENSEARCH_VERSION}

RUN python3 -m venv $HOME/local/python3 && echo 'source $HOME/local/python3/bin/activate' >> $HOME/.profile

RUN mkdir -p /var/lib/minio/data && chown -R nobody:nogroup /var/lib/minio

# https://www.rabbitmq.com/access-control.html#loopback-users
RUN echo 'loopback_users = none' | tee -a /etc/rabbitmq/rabbitmq.conf \
    && sed -i "s/NODENAME.*/NODENAME=palm/g" /etc/rabbitmq/rabbitmq-env.conf \
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
ENV PHP_VERSION="8.3"
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

VOLUME /srv
WORKDIR /srv

CMD ["/bin/bash", "-l"]
