FROM alpine:latest
LABEL maintainer="Jeremy Zheng"

RUN apk update
RUN apk upgrade

RUN apk add bash wget pwgen openssl vim git tzdata rsync \
    build-base patchutils \
    apache2 apache2-ctl apache2-dev fcgiwrap \
    perl perl-dev \
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
RUN cd /var/www/bugzilla \
    && perl install-module.pl Chart::Lines \
    && perl install-module.pl Template::Plugin::GD::Image \
    && perl install-module.pl GD::Text \
    && perl install-module.pl GD::Graph \
    && perl install-module.pl MIME::Parser \
    && perl install-module.pl PatchReader \
    && perl install-module.pl Email::Reply \
    && perl install-module.pl File::MimeInfo::Magic \
    && perl install-module.pl IO::Scalar \
    && perl install-module.pl mod_perl2 \
    && perl install-module.pl Apache2::SizeLimit \
    && perl install-module.pl Encode::Detect \
    && perl install-module.pl Email::Sender \
    && perl install-module.pl Math::Random::ISAAC
RUN mkdir /etc/apache2/sites-available/
COPY etc/apache2.conf /etc/apache2/sites-available/bugzilla.conf
RUN echo 'Include /etc/apache2/sites-available/*.conf' >> /etc/apache2/httpd.conf
COPY etc/run.sh /bugzilla.sh
RUN mkdir /etc/bugzilla
COPY etc/setup.pl /etc/bugzilla/

# https://wiki.mozilla.org/Bugzilla:Installation_with_nginx

RUN echo "$(date -u +%4Y%m%d%H%M%S)" | tee /VERSION

EXPOSE 8080/tcp
VOLUME /workspace
WORKDIR /workspace

CMD ["/bin/bash", "-l"]

