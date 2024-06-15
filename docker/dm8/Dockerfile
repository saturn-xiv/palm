FROM centos:7
LABEL maintainer="Jeremy Zheng"

RUN yum update -y
RUN yum groupinstall -y 'Development Tools'
RUN yum install -y redhat-lsb-core zsh git vim tmux sudo net-tools \
    cmake python3
RUN yum clean all 

RUN useradd -s /bin/zsh -m deploy
RUN passwd -l deploy
RUN echo 'deploy ALL=(ALL:ALL) NOPASSWD: ALL' > /etc/sudoers.d/101-deploy

USER deploy
RUN mkdir -p $HOME/downloads $HOME/build $HOME/local $HOME/tmp

# https://github.com/ohmyzsh/ohmyzsh
RUN git clone https://github.com/ohmyzsh/ohmyzsh.git $HOME/.oh-my-zsh
RUN cp $HOME/.oh-my-zsh/templates/zshrc.zsh-template $HOME/.zshrc
RUN echo 'export LANG=en_US.UTF-8' >> $HOME/.zshrc
RUN echo 'export LC_ALL=en_US.UTF-8' >> $HOME/.zshrc
RUN echo 'export PATH=$HOME/.local/bin:$PATH' >> $HOME/.zshrc

RUN git config --global core.quotepath false \
    && git config --global http.version HTTP/1.1 \
    && git config --global pull.rebase false \
    && git config --global url."https://".insteadOf git://
RUN echo 'set-option -g history-limit 102400' > $HOME/.tmux.conf \
    && echo 'set-option -g default-shell "/bin/zsh"' >> $HOME/.tmux.conf

# Install dm8 server
COPY downloads/DMInstall.bin /
COPY auto-install.xml /etc/dm8-auto-install.xml
RUN sudo mkdir /opt/dmdbms /var/lib/dm8 /opt/bin
RUN sudo chown -R deploy:deploy /opt/dmdbms /var/lib/dm8 /opt/bin
# /DMInstall.bin -i
RUN /DMInstall.bin -q /etc/dm8-auto-install.xml
RUN sed -i '/^systemctl/s//#&/' /opt/dmdbms/script/root/root_installer.sh
RUN sudo /opt/dmdbms/script/root/root_installer.sh
RUN cp /opt/dmdbms/bin/service_template/DmService /opt/bin/DmServiceDMSERVER
RUN sed -i 's/INI_PATH=%INI_PATH%/INI_PATH="\/var\/lib\/dm8\/DAMENG\/dm.ini"/g' /opt/bin/DmServiceDMSERVER

RUN echo "$(date -u +%4Y%m%d%H%M%S)" | sudo tee /VERSION

EXPOSE 5236/tcp

VOLUME /workspace
WORKDIR /workspace

CMD ["/bin/zsh", "-l"]
