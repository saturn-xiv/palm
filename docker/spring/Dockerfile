FROM archlinux:latest
LABEL maintainer="Jeremy Zheng"

RUN pacman-key --init \
    && pacman -Sy --noconfirm archlinux-keyring \
    && pacman-key --populate \
    && pacman -Syu --noconfirm

RUN pacman bzip2 bzip3 p7zip unarchiver \
        git subversion screen vim wget zsh man-db man-pages \
        pwgen sshpass openssl openssh rsync zip unzip tree tmux asciidoc doxygen cpio net-tools bind-tools \
    -S --needed --noconfirm && pacman -Scc --noconfirm
RUN pacman imagemagick ffmpeg xorg-font-util \
        pandoc graphviz \
        hspell nuspell libvoikko hunspell hunspell-en_us \
    -S --needed --noconfirm && pacman -Scc --noconfirm
RUN pacman base-devel clang llvm lld lldb mold bison flex ninja bazel \
        cmake thrift capnproto grpc flatbuffers \
    -S --needed --noconfirm && pacman -Scc --noconfirm
RUN pacman aarch64-linux-gnu-gcc risc-v \
    -S --needed --noconfirm && pacman -Scc --noconfirm
RUN pacman xdebug php-fpm php-pgsql php-sqlite php-redis php-mongodb php-imagick php-gd php-intl php-enchant php-snmp php-tidy php-xsl php-sodium php-odbc \
        libxcrypt-compat \
    -S --needed --noconfirm && pacman -Scc --noconfirm
RUN pacman dart haxe lua erlang elixir jdk-openjdk \
    -S --needed --noconfirm && pacman -Scc --noconfirm
RUN pacman nginx mariadb postgresql rabbitmq redis dovecot postfix supervisor \
        ansible ansible-lint hugo \
    -S --needed --noconfirm && pacman -Scc --noconfirm
RUN pacman wqy-bitmapfont wqy-microhei wqy-zenhei \
        ttf-arphic-ukai ttf-arphic-uming \
        ttf-fira-code woff-fira-code woff2-fira-code \
        ttf-dejavu ttf-ubuntu-font-family \
        adobe-source-code-pro-fonts adobe-source-serif-fonts adobe-source-sans-fonts \
        adobe-source-han-serif-cn-fonts adobe-source-han-serif-hk-fonts adobe-source-han-serif-jp-fonts adobe-source-han-serif-kr-fonts adobe-source-han-serif-tw-fonts \
        adobe-source-han-sans-cn-fonts adobe-source-han-sans-hk-fonts adobe-source-han-sans-jp-fonts adobe-source-han-sans-kr-fonts adobe-source-han-sans-tw-fonts \
     -S --needed --noconfirm && pacman -Scc --noconfirm
RUN pacman texlive -S --needed --noconfirm && pacman -Scc --noconfirm
RUN pacman texlive-lang -S --needed --noconfirm && pacman -Scc --noconfirm
RUN pacman nerd-fonts -S --needed --noconfirm && pacman -Scc --noconfirm

RUN echo "en_US.UTF-8 UTF-8" > /etc/locale.gen && locale-gen
RUN useradd -s /bin/zsh -m deploy
RUN passwd -l deploy
RUN echo 'deploy ALL=(ALL:ALL) NOPASSWD: ALL' > /etc/sudoers.d/101-deploy
USER deploy
RUN mkdir -p $HOME/downloads $HOME/build $HOME/local $HOME/tmp

# https://pkl-lang.org/main/current/pkl-cli/index.html#linux-executable
ENV PKL_VERSION="0.30.2"
RUN curl -L -o $HOME/downloads/pkl https://github.com/apple/pkl/releases/download/$PKL_VERSION/pkl-linux-amd64 \
    && sudo cp $HOME/downloads/pkl /usr/local/bin/ \
    && sudo chmod +x /usr/local/bin/pkl


# https://github.com/envoyproxy/envoy
ENV ENVOY_VERSION="1.38.0"
RUN wget -q -O $HOME/downloads/envoy https://github.com/envoyproxy/envoy/releases/download/v${ENVOY_VERSION}/envoy-${ENVOY_VERSION}-linux-x86_64 \
    && sudo cp $HOME/downloads/envoy /usr/local/bin/ \
    && sudo chmod +x /usr/local/bin/envoy

# https://getcomposer.org/doc/00-intro.md#installation-linux-unix-macos
RUN wget -q -O $HOME/downloads/composer https://getcomposer.org/installer \
    && cd $HOME/downloads \
    && php composer \
    && sudo cp composer.phar /usr/local/bin/composer
RUN echo 'export PATH=$HOME/.config/composer/vendor/bin:$PATH' >> $HOME/.zshrc

# https://github.com/bazelbuild/bazelisk
ENV BAZEL_VERSION="v1.29.0"
RUN wget -q -O $HOME/downloads/bazel https://github.com/bazelbuild/bazelisk/releases/download/${BAZEL_VERSION}/bazelisk-linux-amd64 \
    && sudo cp $HOME/downloads/bazel /usr/local/bin && sudo chmod +x /usr/local/bin/bazel


# https://min.io/download#/linux
RUN wget -q -P $HOME/downloads/ https://dl.min.io/aistor/minio/release/linux-amd64/minio \
    && chmod +x $HOME/downloads/minio \
    && sudo cp $HOME/downloads/minio /usr/local/bin/ \
    && wget -q -P $HOME/downloads/ https://dl.min.io/aistor/mc/release/linux-amd64/mc \
    && chmod +x $HOME/downloads/mc \
    && sudo cp $HOME/downloads/mc /usr/local/bin/

# https://github.com/amacneil/dbmate
RUN curl -fsSL -o $HOME/downloads/dbmate https://github.com/amacneil/dbmate/releases/latest/download/dbmate-linux-amd64 \
    && sudo cp $HOME/downloads/dbmate /usr/local/bin/ \
    && sudo chmod +x /usr/local/bin/dbmate

# https://github.com/fullstorydev/grpcurl?tab=readme-ov-file#from-source
ENV GRPCURL_VERSION="1.9.3"
RUN wget -q -P $HOME/downloads https://github.com/fullstorydev/grpcurl/releases/download/v${GRPCURL_VERSION}/grpcurl_${GRPCURL_VERSION}_linux_x86_64.tar.gz \
    && tar xf $HOME/downloads/grpcurl_${GRPCURL_VERSION}_linux_x86_64.tar.gz -C $HOME/downloads \
    && sudo cp $HOME/downloads/grpcurl /usr/local/bin/ \
    && rm $HOME/downloads/grpcurl 

# https://aur.archlinux.org/packages/ttf-arphic-tex-extra
ENV ARPHIC_TTF_VERSION="20161212-1"
RUN git clone https://aur.archlinux.org/ttf-arphic-tex-extra.git $HOME/downloads/ttf-arphic-tex-extra \
    && cd $HOME/downloads/ttf-arphic-tex-extra/ \
    && makepkg \
    && sudo pacman -U --noconfirm ttf-arphic-tex-extra-${ARPHIC_TTF_VERSION}-any.pkg.tar.zst

# https://github.com/ohmyzsh/ohmyzsh
RUN git clone https://github.com/ohmyzsh/ohmyzsh.git $HOME/.oh-my-zsh \
    && cp $HOME/.oh-my-zsh/templates/zshrc.zsh-template $HOME/.zshrc

RUN echo 'export LANG=en_US.UTF-8' >> $HOME/.zshrc \
    && echo 'export LC_ALL=en_US.UTF-8' >> $HOME/.zshrc \
    && echo 'export EDITOR=vim' >> $HOME/.zshrc \
    && echo 'export PATH=$HOME/.local/bin:$PATH' >> $HOME/.zshrc

RUN git config --global core.quotepath false \
    && git config --global http.version HTTP/1.1 \
    && git config --global pull.rebase false \
    && git config --global url."https://".insteadOf git://

RUN echo 'set-option -g history-limit 102400' > $HOME/.tmux.conf \
    && echo 'set-option -g default-shell "/bin/zsh"' >> $HOME/.tmux.conf

# https://github.com/cross-tools/musl-cross
RUN git clone https://github.com/cross-tools/musl-cross.git $HOME/build/musl-cross
ENV MUSL_TOOLCHAIN_VERSION=20250929
RUN wget -q -P $HOME/downloads https://github.com/cross-tools/musl-cross/releases/download/${MUSL_TOOLCHAIN_VERSION}/x86_64-unknown-linux-musl.tar.xz \
    && tar xf $HOME/downloads/x86_64-unknown-linux-musl.tar.xz -C $HOME/local \
    && echo 'export PATH=$HOME/local/x86_64-unknown-linux-musl/bin:$PATH' >> $HOME/.zshrc
RUN wget -q -P $HOME/downloads https://github.com/cross-tools/musl-cross/releases/download/${MUSL_TOOLCHAIN_VERSION}/aarch64-unknown-linux-musl.tar.xz \
    && tar xf $HOME/downloads/aarch64-unknown-linux-musl.tar.xz -C $HOME/local \
    && echo 'export PATH=$HOME/local/aarch64-unknown-linux-musl/bin:$PATH' >> $HOME/.zshrc
RUN wget -q -P $HOME/downloads https://github.com/cross-tools/musl-cross/releases/download/${MUSL_TOOLCHAIN_VERSION}/riscv64-unknown-linux-musl.tar.xz \
    && tar xf $HOME/downloads/riscv64-unknown-linux-musl.tar.xz -C $HOME/local \
    && echo 'export PATH=$HOME/local/x86_64-linux-musl-cross/bin:$PATH' >> $HOME/.zshrc
RUN wget -q -P $HOME/downloads https://github.com/cross-tools/musl-cross/releases/download/${MUSL_TOOLCHAIN_VERSION}/loongarch64-unknown-linux-musl.tar.xz \
    && tar xf $HOME/downloads/loongarch64-unknown-linux-musl.tar.xz -C $HOME/local \
    && echo 'export PATH=$HOME/local/loongarch64-unknown-linux-musl/bin:$PATH' >> $HOME/.zshrc

# https://github.com/asdf-vm/asdf
# asdf plugin list --urls
# asdf plugin list all
# asdf list all XXX
ENV ASDF_VERSION="0.19.0"
RUN wget -q -P $HOME/downloads https://github.com/asdf-vm/asdf/releases/download/v${ASDF_VERSION}/asdf-v${ASDF_VERSION}-linux-amd64.tar.gz \
    && tar xf $HOME/downloads/asdf-v${ASDF_VERSION}-linux-amd64.tar.gz -C $HOME/downloads \
    && sudo cp $HOME/downloads/asdf /usr/local/bin/ \
    && rm $HOME/downloads/asdf \
    && echo 'export PATH=${ASDF_DATA_DIR:-$HOME/.asdf}/shims:$PATH' >> $HOME/.zshrc

RUN zsh -c "source $HOME/.zshrc && asdf plugin add erlang && asdf install erlang 28.5 && asdf set --home erlang 28.5"
RUN zsh -c "source $HOME/.zshrc && asdf plugin add elixir && asdf install elixir 1.19.5-otp-28 && asdf set --home elixir 1.19.5-otp-28"
RUN echo "export HEX_HTTP_CONCURRENCY=1" >> $HOME/.zshrc && echo "export HEX_HTTP_TIMEOUT=120" >> $HOME/.zshrc
# https://thrift.apache.org/lib/java.html
RUN zsh -c "source $HOME/.zshrc && asdf plugin add java && asdf install java corretto-25.0.3.9.1 && asdf set --home java corretto-25.0.3.9.1"
# https://docs.gradle.org/current/userguide/compatibility.html
RUN zsh -c "source $HOME/.zshrc && asdf plugin add gradle && asdf install gradle 9.5.0 && asdf set --home gradle 9.5.0"
RUN zsh -c "source $HOME/.zshrc && asdf plugin add maven && asdf install maven latest && asdf set --home maven latest"

RUN zsh -c "source $HOME/.zshrc && asdf plugin add golang && asdf install golang latest && asdf set --home golang latest"
# https://github.com/asdf-community/asdf-golang#use
RUN echo '. ${ASDF_DATA_DIR:-$HOME/.asdf}/plugins/golang/set-env.zsh' >> $HOME/.zshrc \
    && echo 'export PATH="$(go env GOBIN):$PATH"' >> $HOME/.zshrc
# https://code.visualstudio.com/docs/languages/go
RUN zsh -c "source $HOME/.zshrc \
    && go install golang.org/x/tools/gopls@latest \
    && go install github.com/go-delve/delve/cmd/dlv@latest \
    && go install honnef.co/go/tools/cmd/staticcheck@latest"
# https://gohugo.io/installation/linux/#build-from-source
RUN zsh -c "source $HOME/.zshrc && CGO_ENABLED=1 go install -tags extended github.com/gohugoio/hugo@latest"
# https://grpc.io/docs/languages/go/quickstart/
RUN zsh -c "source $HOME/.zshrc \
    && go install google.golang.org/protobuf/cmd/protoc-gen-go@latest \
    && go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest"

RUN zsh -c "source $HOME/.zshrc && asdf plugin add kotlin && asdf install kotlin latest && asdf set --home kotlin latest"

RUN zsh -c "source $HOME/.zshrc && asdf plugin add nodejs && asdf install nodejs latest && asdf set --home nodejs latest"
RUN zsh -c "source $HOME/.zshrc && npm install -g yarn"
# https://github.com/grpc/grpc-node
# https://github.com/grpc/grpc-node/tree/master/examples/helloworld/static_codegen
RUN zsh -c "source $HOME/.zshrc && npm install -g grpc-tools"

RUN zsh -c "source $HOME/.zshrc && asdf plugin add ruby && asdf install ruby latest && asdf set --home ruby latest"
RUN zsh -c "source $HOME/.zshrc && gem install bundler"
RUN zsh -c "source $HOME/.zshrc && asdf plugin add python && asdf install python latest && asdf set --home python latest"

# https://pip.pypa.io/en/stable/installation/
RUN zsh -c ". $HOME/.zshrc \
    && pip install --upgrade pip \
    && pip install ansible paramiko conan cmake ninja \
    && pip install grpcio-tools"
RUN echo 'export ANSIBLE_HOST_KEY_CHECKING=False' >> $HOME/.zshrc \
    && echo 'alias peony="ANSIBLE_LOG_PATH=$HOME/tmp/$(date +%Y%m%d%H%M%S).log ansible-playbook"' >> $HOME/.zshrc

# https://www.rust-lang.org/tools/install
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
# https://doc.rust-lang.org/nightly/rustc/platform-support.html
RUN zsh -c "source $HOME/.cargo/env \
    && rustup component add rust-analyzer \
    && rustup target add armv7-unknown-linux-gnueabihf \
    && rustup target add aarch64-unknown-linux-gnu \
    && rustup target add x86_64-unknown-linux-musl \
    && rustup target add aarch64-unknown-linux-musl \
    && rustup target add loongarch64-unknown-linux-gnu"
RUN zsh -c "source $HOME/.zshrc \
    && cargo install diesel_cli \
    && cargo install mdbook \
    && cargo install cross"

# FIXME Unable to find release information from file /etc/os-release
# https://www.swift.org/install/linux/
# RUN wget -q -P $HOME/downloads/ https://download.swift.org/swiftly/linux/swiftly-x86_64.tar.gz \
#     && mkdir -p $HOME/local/swift \
#     && tar xf $HOME/downloads/swiftly-x86_64.tar.gz -C $HOME/local/swift \
#     && cd $HOME/local/swift/ \
#     && ./swiftly init --quiet-shell-followup -y \
#     && echo '. "${SWIFTLY_HOME_DIR:-$HOME/.local/share/swiftly}/env.sh"' >> $HOME/.zshrc


# https://xmake.io/#/guide/installation?id=via-curl
RUN curl -fsSL https://xmake.io/shget.text | bash

# https://github.com/microsoft/vcpkg
RUN echo 'export VCPKG_DISABLE_METRICS=1' >> $HOME/.zshrc

# https://opensearch.org/downloads.html#opensearch
# https://github.com/aparo/opensearch-analysis-ik?tab=readme-ov-file#install
# https://github.com/aparo/opensearch-analysis-stconvert?tab=readme-ov-file#installation
# https://github.com/aparo/opensearch-analysis-pinyin?tab=readme-ov-file#installation
ENV OPENSEARCH_VERSION="3.0.0"
RUN wget -q -P $HOME/downloads \
    https://artifacts.opensearch.org/releases/bundle/opensearch/${OPENSEARCH_VERSION}/opensearch-${OPENSEARCH_VERSION}-linux-x64.tar.gz
RUN tar xf $HOME/downloads/opensearch-${OPENSEARCH_VERSION}-linux-x64.tar.gz -C $HOME/tmp \
    && sudo mv $HOME/tmp/opensearch-${OPENSEARCH_VERSION} /opt/opensearch \
    && cd /opt/opensearch/ \
    && bin/opensearch-plugin install https://github.com/aparo/opensearch-analysis-ik/releases/download/${OPENSEARCH_VERSION}/opensearch-analysis-ik.zip \
    && bin/opensearch-plugin install https://github.com/aparo/opensearch-analysis-stconvert/releases/download/${OPENSEARCH_VERSION}/opensearch-analysis-stconvert.zip \
    && bin/opensearch-plugin install https://github.com/aparo/opensearch-analysis-pinyin/releases/download/${OPENSEARCH_VERSION}/opensearch-analysis-pinyin.zip

# https://github.com/grpc/grpc
ENV GRPC_VERSION="v1.80.0"
RUN git clone -b $GRPC_VERSION https://github.com/grpc/grpc.git $HOME/downloads/grpc
RUN cd $HOME/downloads/grpc/ && git submodule update --init --recursive
# ENV PROTOBUF_VERSION "v3.21.8"
# RUN cd $HOME/downloads/grpc/third_party/protobuf \
#     && git checkout ${PROTOBUF_VERSION} \
#     && git submodule update --init --recursive
RUN zsh -c "source $HOME/.zshrc \
    && mkdir -pv $HOME/build/grpc \
    && cmake -DCMAKE_BUILD_TYPE=Release -G Ninja \
        -DgRPC_INSTALL=ON -DgRPC_SSL_PROVIDER=package -DgRPC_BUILD_TESTS=OFF \
        -DCMAKE_INSTALL_PREFIX=$HOME/.local -B $HOME/build/grpc -S $HOME/downloads/grpc \
    && cmake --build $HOME/build/grpc \
    && cmake --install $HOME/build/grpc \
    && rm -r $HOME/build/grpc"

# https://github.com/grpc/grpc-web#code-generator-plugin
ENV GRPC_WEB_PLUGIN_VERSION="2.0.2"
RUN wget -q -O $HOME/.local/bin/protoc-gen-grpc-web \
    https://github.com/grpc/grpc-web/releases/download/${GRPC_WEB_PLUGIN_VERSION}/protoc-gen-grpc-web-${GRPC_WEB_PLUGIN_VERSION}-linux-x86_64
RUN chmod +x $HOME/.local/bin/protoc-gen-grpc-web

# https://github.com/protocolbuffers/protobuf-javascript
ENV GRPC_JS_PLUGIN_VERSION="4.0.2"
RUN wget -q -P $HOME/downloads \
    https://github.com/protocolbuffers/protobuf-javascript/releases/download/v${GRPC_JS_PLUGIN_VERSION}/protobuf-javascript-${GRPC_JS_PLUGIN_VERSION}-linux-x86_64.tar.gz
RUN mkdir -p $HOME/build/protobuf-javascript \
    && tar xf $HOME/downloads/protobuf-javascript-${GRPC_JS_PLUGIN_VERSION}-linux-x86_64.tar.gz -C $HOME/build/protobuf-javascript \
    && cp $HOME/build/protobuf-javascript/bin/protoc-gen-js $HOME/.local/bin/

# https://repo1.maven.org/maven2/io/grpc/protoc-gen-grpc-java/
# https://github.com/grpc/grpc-java
ENV GRPC_JAVA_PLUGIN_VERSION="1.81.0"
RUN wget -q -O $HOME/.local/bin/grpc_java_plugin \
    https://repo1.maven.org/maven2/io/grpc/protoc-gen-grpc-java/${GRPC_JAVA_PLUGIN_VERSION}/protoc-gen-grpc-java-${GRPC_JAVA_PLUGIN_VERSION}-linux-x86_64.exe
RUN chmod +x $HOME/.local/bin/grpc_java_plugin

# https://github.com/google/flatbuffers
ENV FLATBUFFERS_VERSION="v25.12.19-2026-02-06-03fffb2"
RUN git clone -b $FLATBUFFERS_VERSION https://github.com/google/flatbuffers.git $HOME/downloads/flatbuffers
RUN zsh -c "source $HOME/.zshrc \
    && mkdir -pv $HOME/build/flatbuffers \
    && CC=clang CXX=clang++ cmake -G Ninja -DCMAKE_BUILD_TYPE=Release \
        -DFLATBUFFERS_BUILD_TESTS=OFF \
        -DCMAKE_INSTALL_PREFIX=$HOME/.local -B $HOME/build/flatbuffers -S $HOME/downloads/flatbuffers \
    && cmake --build $HOME/build/flatbuffers \
    && cmake --install $HOME/build/flatbuffers \
    && rm -r $HOME/build/flatbuffers"

# https://opensearch.org/docs/latest/opensearch/install/tar/
# https://docs.opensearch.org/latest/install-and-configure/plugins/#list
RUN cd /opt/opensearch/ \
    && echo "network.host: 0.0.0.0" >> config/opensearch.yml \
    && echo "discovery.type: single-node" >> config/opensearch.yml \
    && echo "plugins.security.disabled: true" >> config/opensearch.yml \
    && mkdir -p config/opensearch-performance-analyzer \
    && touch config/opensearch-performance-analyzer/plugin-stats-metadata \
    && cp -r config/opensearch-analysis-ik plugins/opensearch-analysis-ik/config

RUN sudo mkdir -p /var/lib/minio/data && sudo chown -R nobody:nobody /var/lib/minio

# https://www.rabbitmq.com/access-control.html#loopback-users
RUN echo 'loopback_users = none' | sudo tee -a /etc/rabbitmq/rabbitmq.conf \
    && sudo sed -i "s/NODENAME.*/NODENAME=palm/g" /etc/rabbitmq/rabbitmq-env.conf \
    && sudo rabbitmq-plugins enable rabbitmq_mqtt \
    && sudo rabbitmq-plugins enable rabbitmq_management \
    && sudo rabbitmq-plugins enable rabbitmq_stream \
    && sudo rabbitmq-plugins enable rabbitmq_shovel rabbitmq_shovel_management

RUN cd /usr && sudo mariadb-install-db --datadir=/var/lib/mysql --user=mysql

RUN sudo -s -u postgres /usr/bin/initdb -D /var/lib/postgres/data
RUN echo "listen_addresses = '0.0.0.0'" | sudo tee -a /var/lib/postgres/data/postgresql.conf \
    && echo "host  all  all 0.0.0.0/0 md5" | sudo tee -a /var/lib/postgres/data/pg_hba.conf \
    && sudo mkdir /run/postgresql \
    && sudo chown -R postgres:postgres /run/postgresql

ADD etc/redis/* /etc/redis/
RUN sudo mkdir -p /var/lib/redis \
    && sudo mkdir /var/lib/redis/single /var/lib/redis/node-1 /var/lib/redis/node-2 /var/lib/redis/node-3 /var/lib/redis/node-4 /var/lib/redis/node-5 /var/lib/redis/node-6 \
    && sudo chown -R valkey:valkey /var/lib/redis \
    && sudo chmod 750 /var/lib/redis

RUN sudo mkdir -p /var/www/html \
    && echo "<?php phpinfo(); ?>" | sudo tee /var/www/html/info.php \
    && sudo cp /etc/nginx/nginx.conf /etc/nginx/nginx.conf.orig \
    && echo "daemon off;" | sudo tee -a /etc/nginx/nginx.conf \
    && sudo sed -i '35,82d' /etc/nginx/nginx.conf \
    && sudo sed -i '35i include sites-enabled/*.conf;' /etc/nginx/nginx.conf \
    && sudo sed -i "s/error_log =.*/error_log = log\/php-fpm.log/g" /etc/php/php-fpm.conf
ADD etc/nginx/sites-enabled/localhost.conf /etc/nginx/sites-enabled/localhost.conf

# php -i
# <?php php_info(); ?>
# <?php xdebug_info(); ?>
RUN sudo sed -i 's/memory_limit =.*/memory_limit = 1G/' /etc/php/php.ini \
    && sudo sed -i 's/;extension=bcmath/extension=bcmath/' /etc/php/php.ini \
    && sudo sed -i 's/;extension=bz2/extension=bz2/' /etc/php/php.ini \
    && sudo sed -i 's/;extension=calendar/extension=calendar/' /etc/php/php.ini \
    && sudo sed -i 's/;extension=enchant/extension=enchant/' /etc/php/php.ini \
    && sudo sed -i 's/;extension=exif/extension=exif/' /etc/php/php.ini \
    && sudo sed -i 's/;extension=ftp/extension=ftp/' /etc/php/php.ini \
    && sudo sed -i 's/;extension=gd/extension=gd/' /etc/php/php.ini \
    && sudo sed -i 's/;extension=gettext/extension=gettext/' /etc/php/php.ini \
    && sudo sed -i 's/;extension=gmp/extension=gmp/' /etc/php/php.ini \
    && sudo sed -i 's/;extension=iconv/extension=iconv/' /etc/php/php.ini \
    && sudo sed -i 's/;extension=intl/extension=intl/' /etc/php/php.ini \
    && sudo sed -i 's/;extension=mysqli/extension=mysqli/' /etc/php/php.ini \
    && sudo sed -i 's/;extension=odbc/extension=odbc/' /etc/php/php.ini \
    && sudo sed -i 's/;extension=pdo_mysql/extension=pdo_mysql/' /etc/php/php.ini \
    && sudo sed -i 's/;extension=pdo_odbc/extension=pdo_odbc/' /etc/php/php.ini \
    && sudo sed -i 's/;extension=pdo_pgsql/extension=pdo_pgsql/' /etc/php/php.ini \
    && sudo sed -i 's/;extension=pdo_sqlite/extension=pdo_sqlite/' /etc/php/php.ini \
    && sudo sed -i 's/;extension=pgsql/extension=pgsql/' /etc/php/php.ini \
    && sudo sed -i 's/;extension=snmp/extension=snmp/' /etc/php/php.ini \
    && sudo sed -i 's/;extension=soap/extension=soap/' /etc/php/php.ini \
    && sudo sed -i 's/;extension=sockets/extension=sockets/' /etc/php/php.ini \
    && sudo sed -i 's/;extension=sodium/extension=sodium/' /etc/php/php.ini \
    && sudo sed -i 's/;extension=sqlite3/extension=sqlite3/' /etc/php/php.ini \
    && sudo sed -i 's/;extension=tidy/extension=tidy/' /etc/php/php.ini \
    && sudo sed -i 's/;extension=xsl/extension=xsl/' /etc/php/php.ini \
    && sudo sed -i 's/;extension=redis/extension=redis/' /etc/php/conf.d/redis.ini \
    && sudo sed -i 's/;extension=igbinary/extension=igbinary/' /etc/php/conf.d/igbinary.ini \
    && sudo sed -i 's/; extension = imagick/extension=imagick/' /etc/php/conf.d/imagick.ini \
    && sudo sed -i 's/;zend_extension=opcache/zend_extension=opcache/' /etc/php/php.ini \
    && sudo sed -i 's/;zend_extension=xdebug/zend_extension=xdebug/' /etc/php/conf.d/xdebug.ini \
    && sudo sed -i 's/;xdebug.mode=debug/xdebug.mode=develop,coverage,debug/' /etc/php/conf.d/xdebug.ini \
    && echo 'xdebug.start_with_request=trigger' | sudo tee -a /etc/php/conf.d/xdebug.ini \
    && echo 'xdebug.client_host=0.0.0.0' | sudo tee -a /etc/php/conf.d/xdebug.ini \
    && echo 'xdebug.client_port=9003' | sudo tee -a /etc/php/conf.d/xdebug.ini \
    && echo 'xdebug.cli_color = 2' | sudo tee -a /etc/php/conf.d/xdebug.ini

# && sudo sed -i 's/;xdebug.mode=debug/xdebug.mode=develop,profile,coverage,debug/' /etc/php/conf.d/xdebug.ini \

RUN mkdir -p $HOME/.ssh \
    && touch $HOME/.ssh/authorized_keys \
    && echo 'GSSAPIAuthentication no' | sudo tee -a /etc/ssh/sshd_config \
    && echo 'UseDNS no' | sudo tee -a /etc/ssh/sshd_config \
    && echo 'PasswordAuthentication no' | sudo tee -a /etc/ssh/sshd_config \
    && echo 'PermitRootLogin no' | sudo tee -a /etc/ssh/sshd_config \
    && echo 'AllowUsers deploy' | sudo tee -a /etc/ssh/sshd_config \
    && echo 'PrintLastLog no' | sudo tee -a /etc/ssh/sshd_config \
    && sudo /usr/bin/ssh-keygen -A

ADD etc/envoy.yaml /etc/
RUN sudo mkdir -p /var/log/envoy
ADD etc/supervisor /etc/supervisor
ADD opt/bin /opt/bin

RUN echo "$(date -u +%4Y%m%d%H%M%S)" | sudo tee /VERSION

VOLUME /mnt
WORKDIR /mnt

CMD ["/bin/zsh", "-l"]
