FROM amazonlinux:2016.09.0.20161028
LABEL maintainer="Jeremy Zheng"

RUN yum update -y
RUN yum upgrade -y
RUN yum groupinstall -y 'Development Tools'
RUN yum install -y which shadow-utils passwd sudo git vim net-tools rsync screen wget curl \
    cmake gcc72 gcc72-c++ python38 \
    tcl perl-IPC-Cmd libffi-devel zlib-devel sqlite-devel libuuid-devel
RUN yum clean all


RUN localedef -i en_US -c -f UTF-8 en_US.UTF-8

ENV LC_ALL=en_US.utf-8
ENV LANG=en_US.utf-8

RUN useradd -s /bin/bash -m deploy
RUN passwd -l deploy
RUN echo 'deploy ALL=(ALL:ALL) NOPASSWD: ALL' > /etc/sudoers.d/101-deploy

USER deploy
RUN mkdir -p $HOME/downloads $HOME/build $HOME/local $HOME/tmp

# https://packaging.python.org/en/latest/tutorials/installing-packages/#ensure-you-can-run-pip-from-the-command-line
RUN wget -q -P $HOME/downloads https://bootstrap.pypa.io/get-pip.py
RUN python3 $HOME/downloads/get-pip.py --user
# https://github.com/codrsquad/portable-python?tab=readme-ov-file#installation
RUN $HOME/.local/bin/pip install --user portable-python

# https://openssl-library.org/source/old/1.1.1/index.html
ENV OPENSSL_VERSION "1.1.1w"
RUN wget -q -P $HOME/downloads https://github.com/openssl/openssl/releases/download/OpenSSL_1_1_1w/openssl-${OPENSSL_VERSION}.tar.gz
RUN tar xf $HOME/downloads/openssl-${OPENSSL_VERSION}.tar.gz -C $HOME/build

# https://www.sqlite.org/download.html
ENV SQLITE3_VERSION "3490100"
RUN wget -q -P $HOME/downloads https://www.sqlite.org/2025/sqlite-src-${SQLITE3_VERSION}.zip
RUN unzip -q $HOME/downloads/sqlite-src-${SQLITE3_VERSION}.zip -d $HOME/build

# python38-devel \
#     libffi-devel zlib-devel \
#     xz-devel bzip2-devel readline-devel sqlite-devel ncurses-devel gdbm-devel perl-IPC-Cmd openssl-devel uuid-devel
# RUN cd $HOME/build/openssl-1.1.1w \
#     && ./config --prefix=/usr --openssldir=/etc/ssl --libdir=lib no-shared zlib-dynamic \
#     && make \
#     && make install
# RUN echo 'export PKG_CONFIG_PATH=/usr/local/lib64/pkgconfig:$PKG_CONFIG_PATH' >> $HOME/.bash_profile

RUN curl -fsSL https://pyenv.run | bash
RUN echo 'export PYENV_ROOT="$HOME/.pyenv"' >> $HOME/.bash_profile
RUN echo '[[ -d $PYENV_ROOT/bin ]] && export PATH="$PYENV_ROOT/bin:$PATH"' >> $HOME/.bash_profile
RUN echo 'eval "$(pyenv init - bash)"' >> $HOME/.bash_profile


VOLUME /workspace
WORKDIR /workspace

CMD ["/bin/bash", "-l"]
