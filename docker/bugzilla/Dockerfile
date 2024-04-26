FROM alpine:latest
LABEL maintainer="Jeremy Zheng"

RUN apk update
RUN apk upgrade

RUN apk add bash wget pwgen openssl git tzdata rsync \
    build-base apache2 apache2-ctl fcgiwrap perl \
    perl-cgi perl-date-format perl-datetime perl-datetime-timezone \
    perl-template-toolkit perl-email-mime perl-uri perl-list-moreutils perl-json-xs \
    perl-dbi perl-dbd-sqlite perl-dbd-pg perl-dbd-mysql \
    perl-yaml perl-cpan-distnameinfo perl-params-util \
    perl-net-smtp-ssl perl-email-address perl-class-xsaccessor \
    perl-gd perl-html-scrubber perl-html-formattext-withlinks perl-xml-twig \
    perl-file-copy-recursive perl-file-which perl-file-mimeinfo
ENV TZ=UTC
RUN cp /usr/share/zoneinfo/UTC /etc/localtime

RUN mkdir -p $HOME/downloads $HOME/local $HOME/tmp

# https://bugzilla.readthedocs.io/en/5.2/installing/quick-start.html#download-bugzilla
RUN git clone -b release-5.0.6 https://github.com/bugzilla/bugzilla.git /var/www/bugzilla
# ./checksetup.pl
# Template::Plugin::GD::Image Encode::Detect
RUN perl install-module.pl Chart::Lines \
    && perl install-module.pl GD::Text \
    && perl install-module.pl GD::Graph \
    && perl install-module.pl MIME::Parser \
    && perl install-module.pl PatchReader \
    && perl install-module.pl Email::Reply \
    && perl install-module.pl File::MimeInfo::Magic \
    && perl install-module.pl IO::Scalar \
    && perl install-module.pl Email::Sender \
    && perl install-module.pl Math::Random::ISAAC
RUN mkdir /etc/apache2/sites-available/
COPY apache2.conf /etc/apache2/sites-available/bugzilla.conf
RUN echo 'Include /etc/apache2/sites-available/*.conf' >> /etc/apache2/httpd.conf
RUN echo 'ServerName localhost' >> /etc/apache2/httpd.conf
RUN echo 'ErrorLog /workspace/tmp/error.log' >> /etc/apache2/httpd.conf
RUN echo 'CustomLog /workspace/tmp/access.log' combined >> /etc/apache2/httpd.conf
RUN tee >>/var/www/bugzilla/localconfig <<EOF
\$site_wide_secret = '$(pwgen 64 1)';
\$webservergroup = "nobody";
\$db_driver = "sqlite";
\$db_name = "bugs";
EOF
COPY run.sh /bugzilla.sh

# https://wiki.mozilla.org/Bugzilla:Installation_with_nginx

RUN echo "$(date -u +%4Y%m%d%H%M%S)" | tee /VERSION

EXPOSE 8080/tcp
VOLUME /workspace
WORKDIR /workspace

CMD ["/bugzilla.sh"]

