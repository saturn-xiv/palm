FROM ubuntu:latest
LABEL maintainer="Jeremy Zheng"

ENV DEBIAN_FRONTEND=noninteractive

RUN apt update
RUN apt -y upgrade
RUN apt install -y lsb-release sudo
RUN apt -y autoremove
RUN apt -y clean

RUN groupadd dinstall
RUN useradd -s /bin/bash -g dinstall -m dmdba
RUN passwd -l dmdba
RUN echo 'dmdba ALL=(ALL:ALL) NOPASSWD: ALL' > /etc/sudoers.d/101-dmdba

USER dmdba
RUN mkdir -p $HOME/downloads $HOME/build $HOME/local $HOME/tmp

# Install dm8 server
COPY downloads/DMInstall.bin downloads/DMInstall.bin.md5 /opt/
RUN sudo chmod 555 /opt/DMInstall.bin
RUN sudo chmod 444 /opt/DMInstall.bin.md5

COPY downloads/dm.key /etc/
RUN sudo chmod 400 /etc/dm.key
RUN sudo chown dmdba:dinstall /etc/dm.key

COPY auto-install.xml /etc/dm8-auto-install.xml
RUN sudo chmod 400 /etc/dm8-auto-install.xml
RUN sudo chown dmdba:dinstall /etc/dm8-auto-install.xml

RUN sudo mkdir /opt/dmdbms /var/lib/dm8 /opt/bin
RUN sudo chown -R dmdba:dinstall /opt/dmdbms /var/lib/dm8 /opt/bin

# /DMInstall.bin -i
# fix ERROR StatusLogger Could not reconfigure JMX
RUN LOG4J_DISABLE_JMX=true /opt/DMInstall.bin -q /etc/dm8-auto-install.xml
RUN sed -i '/^systemctl/s//#&/' /opt/dmdbms/script/root/root_installer.sh
RUN sudo /opt/dmdbms/script/root/root_installer.sh
RUN cp /opt/dmdbms/bin/service_template/DmService /opt/bin/DmServiceDMSERVER
RUN sed -i 's/INI_PATH=%INI_PATH%/INI_PATH="\/var\/lib\/dm8\/DAMENG\/dm.ini"/g' /opt/bin/DmServiceDMSERVER
RUN sed -i '521s/.*/ENABLE_ENCRYPT = 0/' /var/lib/dm8/DAMENG/dm.ini

RUN echo "$(date -u +%4Y%m%d%H%M%S)" | sudo tee /VERSION

EXPOSE 5236/tcp

VOLUME /workspace
WORKDIR /workspace

CMD ["/bin/bash", "-l"]
