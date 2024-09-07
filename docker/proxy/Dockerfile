FROM alpine:latest
LABEL maintainer="Jeremy Zheng"

RUN apk update
RUN apk upgrade
RUN apk add bash wget pwgen openssl git rsync \
    inetutils-telnet bind-tools \
    build-base openssh-client sshpass \
    python3 py3-pip python3-dev 

RUN mkdir -p $HOME/downloads $HOME/local $HOME/tmp

# https://github.com/qwj/python-proxy
RUN python3 -m venv $HOME/local/python3
RUN sh -c ". $HOME/local/python3/bin/activate \
    && pip install asyncssh pproxy[accelerated]"

COPY proxy.sh /opt/
RUN echo "$(date -u +%4Y%m%d%H%M%S)" | tee /VERSION

VOLUME /workspace
WORKDIR /workspace

CMD ["/opt/proxy.sh"]

