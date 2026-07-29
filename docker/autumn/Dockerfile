FROM ubuntu:latest
LABEL maintainer="Jeremy Zheng"

ENV DEBIAN_FRONTEND="noninteractive"

RUN apt update && apt -y upgrade && apt -y clean
RUN apt -y install lsb-release apt-utils debian-keyring debian-archive-keyring apt-transport-https software-properties-common curl wget gnupg \
    && apt -y clean
RUN apt -y install pwgen git locales locales-all vim tzdata zip unzip tree tmux screen \
    curl wget rsync telnet dnsutils netcat-openbsd iputils-ping lftp sudo \
    openssh-client sshpass \
    python3-full \
    && apt -y clean
RUN apt -y install cmake pkg-config libtool automake autoconf autoconf-archive cpio mold bison flex meson ninja-build debhelper \
    build-essential crossbuild-essential-amd64 crossbuild-essential-arm64 crossbuild-essential-armhf crossbuild-essential-riscv64 \
    g++ g++-x86-64-linux-gnu g++-aarch64-linux-gnu g++-riscv64-linux-gnu \
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
RUN apt install -y postgresql redis rabbitmq-server vsftpd nginx certbot sqlite3 \
    && apt -y clean

RUN apt install -y php-fpm \
    php-bz2 php-zip php-xml php-xsl \
    php-bcmath php-gmp php-enchant\
    php-mbstring php-curl php-zmq php-imagick php-gd php-intl php-soap \
    php-odbc php-mysql php-pgsql php-sqlite3 php-redis php-mongodb php-amqp \
    php-snmp \
    && apt -y clean
RUN apt install -y libssl-dev libsodium-dev libpq-dev libmysqlclient-dev libsqlite3-dev libyaml-dev libffi-dev libbz2-dev zlib1g-dev libncurses-dev python3-dev && apt -y clean
# https://apt.llvm.org/
RUN bash -c "$(wget -O - https://apt.llvm.org/llvm.sh)"
RUN apt -y clean && apt -y autoremove

RUN update-alternatives --install /usr/bin/clang clang /usr/bin/clang-22 100
RUN update-alternatives --install /usr/bin/clang++ clang++ /usr/bin/clang++-22 100


RUN echo "en_US.UTF-8 UTF-8" > /etc/locale.gen && locale-gen && update-locale LANG=en_US.UTF-8 && update-locale LANGUAGE=en_US.UTF-8 && update-locale LC_ALL=en_US.UTF-8
RUN update-alternatives --set editor /usr/bin/vim.basic
RUN echo 'ubuntu ALL=(ALL:ALL) NOPASSWD: ALL' > /etc/sudoers.d/101-ubuntu
RUN echo "ubuntu:ubuntu" | chpasswd

USER ubuntu

RUN echo 'export LANG="en_US.UTF-8"' >> $HOME/.profile && echo 'export LANGUAGE="en_US.UTF-8"' >> $HOME/.profile
RUN mkdir -p $HOME/local $HOME/tmp $HOME/downloads $HOME/build

RUN git config --global core.quotepath false \
    && git config --global http.version HTTP/1.1 \
    && git config --global pull.rebase false \
    && git config --global url."https://".insteadOf git://
RUN echo 'set-option -g history-limit 102400' > $HOME/.tmux.conf && echo 'set-option -g default-shell "/bin/bash"' >> $HOME/.tmux.conf
RUN echo 'set nobackup' >>$HOME/.vimrc && echo 'set nowritebackup' >> $HOME/.vimrc

ARG TARGETARCH

# https://min.io/download#/linux
RUN wget -q -P $HOME/downloads/ https://dl.min.io/aistor/minio/release/linux-${TARGETARCH}/minio \
    && chmod +x $HOME/downloads/minio \
    && sudo cp $HOME/downloads/minio /usr/local/bin/ \
    && rm $HOME/downloads/minio \
    && wget -q -P $HOME/downloads/ https://dl.min.io/aistor/mc/release/linux-${TARGETARCH}/mc \
    && chmod +x $HOME/downloads/mc \
    && sudo cp $HOME/downloads/mc /usr/local/bin/ \
    && rm $HOME/downloads/mc

# https://github.com/amacneil/dbmate
RUN curl -fsSL -o $HOME/downloads/dbmate https://github.com/amacneil/dbmate/releases/latest/download/dbmate-linux-${TARGETARCH} \
    && chmod +x $HOME/downloads/dbmate \
    && sudo cp $HOME/downloads/dbmate /usr/local/bin/ \
    && rm $HOME/downloads/dbmate

# https://getcomposer.org/doc/00-intro.md#installation-linux-unix-macos
RUN wget -q -O $HOME/downloads/composer-setup.php https://getcomposer.org/installer \
    && cd $HOME/downloads \
    && sudo php $HOME/downloads/composer-setup.php --install-dir=/usr/local/bin --filename=composer \
    && rm $HOME/downloads/composer-setup.php
RUN echo 'export PATH=$HOME/.config/composer/vendor/bin:$PATH' >> $HOME/.profile

# https://www.rust-lang.org/tools/install
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN bash -c "source $HOME/.cargo/env \
    && rustup target add armv7-unknown-linux-gnueabihf \
    && rustup target add aarch64-unknown-linux-gnu \
    && rustup target add riscv64gc-unknown-linux-gnu \
    && rustup target add loongarch64-unknown-linux-gnu"
RUN bash -c "source $HOME/.cargo/env && cargo install diesel_cli && cargo install mdbook"


# https://github.com/asdf-vm/asdf
ENV ASDF_VERSION="0.19.0"
RUN wget -q -P $HOME/downloads https://github.com/asdf-vm/asdf/releases/download/v${ASDF_VERSION}/asdf-v${ASDF_VERSION}-linux-${TARGETARCH}.tar.gz \
    && tar xf $HOME/downloads/asdf-v${ASDF_VERSION}-linux-${TARGETARCH}.tar.gz -C $HOME/downloads \
    && sudo cp $HOME/downloads/asdf /usr/local/bin/ \
    && echo 'export PATH=${ASDF_DATA_DIR:-$HOME/.asdf}/shims:$PATH' >> $HOME/.profile \
    && rm $HOME/downloads/asdf $HOME/downloads/asdf-v${ASDF_VERSION}-linux-${TARGETARCH}.tar.gz

RUN bash -c "source $HOME/.profile && asdf plugin add erlang && asdf install erlang 29.0.3 && asdf set --home erlang 29.0.3"
RUN bash -c "source $HOME/.profile && asdf plugin add elixir && asdf install elixir 1.20.2-otp-29 && asdf set --home elixir 1.20.2-otp-29"
RUN echo "export HEX_HTTP_CONCURRENCY=1" >> $HOME/.profile && echo "export HEX_HTTP_TIMEOUT=120" >> $HOME/.profile
# https://thrift.apache.org/lib/java.html
RUN bash -c "source $HOME/.profile && asdf plugin add java && asdf install java corretto-25.0.3.9.1 && asdf set --home java corretto-25.0.3.9.1"
# https://docs.gradle.org/current/userguide/compatibility.html
RUN bash -c "source $HOME/.profile && asdf plugin add gradle && asdf install gradle 9.6.1 && asdf set --home gradle 9.6.1"
RUN bash -c "source $HOME/.profile && asdf plugin add maven && asdf install maven latest && asdf set --home maven latest"

RUN bash -c "source $HOME/.profile && asdf plugin add golang && asdf install golang latest && asdf set --home golang latest"
# https://github.com/asdf-community/asdf-golang#use
RUN echo '. ${ASDF_DATA_DIR:-$HOME/.asdf}/plugins/golang/set-env.bash' >> $HOME/.profile && echo 'export PATH="$(go env GOBIN):$PATH"' >> $HOME/.zshrc
# https://code.visualstudio.com/docs/languages/go
RUN bash -c "source $HOME/.profile \
    && go install golang.org/x/tools/gopls@latest \
    && go install github.com/go-delve/delve/cmd/dlv@latest \
    && go install honnef.co/go/tools/cmd/staticcheck@latest"
# https://github.com/fullstorydev/grpcurl#from-source
RUN bash -c "source $HOME/.profile && go install github.com/fullstorydev/grpcurl/cmd/grpcurl@latest"
# https://gohugo.io/installation/linux/#build-from-source
RUN bash -c "source $HOME/.profile && CGO_ENABLED=1 go install -tags extended github.com/gohugoio/hugo@latest"

RUN bash -c "source $HOME/.profile && asdf plugin add kotlin && asdf install kotlin latest && asdf set --home kotlin latest"

RUN bash -c "source $HOME/.profile && asdf plugin add nodejs && asdf install nodejs latest && asdf set --home nodejs latest"
RUN bash -c "source $HOME/.profile && npm install -g yarn"

RUN bash -c "source $HOME/.profile && asdf plugin add ruby && asdf install ruby latest && asdf set --home ruby latest"
RUN bash -c "source $HOME/.profile && gem install bundler"

RUN bash -c "source $HOME/.profile && asdf plugin add python && asdf install python latest && asdf set --home python latest"
RUN bash -c "source $HOME/.profile && pip install --upgrade pip && pip install cmake ninja"

RUN bash -c "source $HOME/.profile && cd ~/ && asdf plugin add zig && asdf install zig latest && asdf set --home zig latest"


# https://xmake.io/guide/quick-start.html
RUN curl -fsSL https://xmake.io/shget.text | bash
RUN echo 'export XMAKE_ROOT=y' >> $HOME/.profile

# https://learn.microsoft.com/en-us/vcpkg/get_started/get-started?pivots=shell-bash
RUN echo 'export VCPKG_DISABLE_METRICS=1' >> $HOME/.profile

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

RUN tar xf $HOME/downloads/opensearch-${OPENSEARCH_VERSION}-linux-${TARGETARCH}.tar.gz -C $HOME/tmp \
    && rm $HOME/downloads/opensearch-${OPENSEARCH_VERSION}-linux-${TARGETARCH}.tar.gz \
    && sudo mv $HOME/tmp/opensearch-${OPENSEARCH_VERSION} /opt/opensearch \
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

# https://www.rabbitmq.com/access-control.html#loopback-users
RUN echo 'loopback_users = none' | sudo tee -a /etc/rabbitmq/rabbitmq.conf \
    && echo 'NODE_IP_ADDRESS=0.0.0.0' |sudo tee -a /etc/rabbitmq/rabbitmq-env.conf \
    && sudo rabbitmq-plugins enable rabbitmq_mqtt \
    && sudo rabbitmq-plugins enable rabbitmq_stream \
    && sudo rabbitmq-plugins enable rabbitmq_shovel rabbitmq_shovel_management

RUN sudo chown www-data:www-data /var/log/nginx

RUN sudo sed -i 's/memory_limit =.*/memory_limit = 1G/' /etc/php/8.5/fpm/php.ini && sudo sed -i 's/^listen = .*/listen = 0.0.0.0:9000/' /etc/php/8.5/fpm/pool.d/www.conf

RUN echo "$(date -u +%4Y%m%d%H%M%S)" | sudo tee /VERSION

EXPOSE 80/tcp 8080/tcp 9000/tcp 5432/tcp 6379/tcp 16379/tcp 5672/tcp 9200/tcp 9300/tcp 9600/tcp
VOLUME /srv
WORKDIR /srv

CMD ["/bin/bash", "-l"]
