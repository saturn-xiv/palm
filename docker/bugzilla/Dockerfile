FROM ubuntu:jammy
LABEL maintainer="Jeremy Zheng"

ENV DEBIAN_FRONTEND noninteractive

RUN apt update
RUN apt -y upgrade
RUN apt install -y git vim libdbd-sqlite3-perl libdbd-pg-perl \
    apache2 build-essential libcgi-pm-perl libdigest-sha-perl libtimedate-perl libdatetime-perl libdatetime-timezone-perl libdbi-perl libdbix-connector-perl libtemplate-perl libemail-address-perl libemail-sender-perl libemail-mime-perl liburi-perl liblist-moreutils-perl libmath-random-isaac-perl libjson-xs-perl libgd-perl libchart-perl libtemplate-plugin-gd-perl libgd-text-perl libgd-graph-perl libmime-tools-perl libwww-perl libxml-twig-perl libnet-ldap-perl libauthen-sasl-perl libnet-smtp-ssl-perl libauthen-radius-perl libsoap-lite-perl libxmlrpc-lite-perl libjson-rpc-perl libtest-taint-perl libhtml-parser-perl libhtml-scrubber-perl libencode-perl libencode-detect-perl libemail-reply-perl libhtml-formattext-withlinks-perl libtheschwartz-perl libdaemon-generic-perl libapache2-mod-perl2 libapache2-mod-perl2-dev libfile-mimeinfo-perl libio-stringy-perl libcache-memcached-perl libfile-copy-recursive-perl libfile-which-perl libdbd-mysql-perl perlmagick lynx graphviz python3-sphinx rst2pdf
RUN apt -y autoremove
RUN apt -y clean

# https://bugzilla.readthedocs.io/en/latest/installing/quick-start.html
RUN git clone -b release-5.0.6 https://github.com/bugzilla/bugzilla.git /var/www/bugzilla
RUN cd /var/www/bugzilla && perl install-module.pl PatchReader
COPY etc/apache2.conf /etc/apache2/sites-available/bugzilla.conf
COPY etc/index.html /var/www/html/index.html
RUN a2ensite bugzilla
RUN a2enmod cgi headers expires rewrite
COPY etc/start.sh etc/backup.sh /
RUN mkdir /etc/bugzilla
COPY etc/setup.pl /etc/bugzilla/

RUN echo "$(date -u +%4Y%m%d%H%M%S)" | tee /VERSION

EXPOSE 80/tcp
VOLUME /workspace
WORKDIR /var/www/bugzilla

CMD ["/bin/bash", "-l"]
