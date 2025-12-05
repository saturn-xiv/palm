#!/bin/bash

set -e

if [ "$EUID" -ne 0 ]; then
    echo "please run this script as root."
    exit 1
fi

. /etc/os-release
if [ "$ID" != "debian" ]; then
    echo "unsupported distribution $ID"
    exit 1
fi

apt update
apt upgrade -y
apt install -y sudo vim tmux pwgen curl bc \
    dnsmasq geoip-bin geoip-database inotify-tools watchman \
    net-tools inetutils-tools sysstat iftop nload nethogs nmap netplan.io firewalld \
    cmake ninja-build git build-essential distcc crossbuild-essential-arm64 crossbuild-essential-riscv64 \
    firmware-misc-nonfree firmware-linux-nonfree \
    crun podman buildah fuse-overlayfs \
    intel-microcode firmware-intel-graphics firmware-intel-misc firmware-intel-sound \
    cups printer-driver-cups-pdf

if [ ! -f /etc/ssh/sshd_config.orig ]; then
    cp /etc/ssh/sshd_config /etc/ssh/sshd_config.orig
    cat >> /etc/ssh/sshd_config <<EOF
# AllowUsers deploy
PermitRootLogin no
PasswordAuthentication no
EOF
fi

systemctl disable dnsmasq
update-alternatives --set editor /usr/bin/vim.basic
# timedatectl list-timezones
timedatectl set-timezone UTC

# https://wiki.debian.org/DebianFirewall
cat > /etc/systemd/system/firewall.service <<EOF
[Unit]
Description=Apply firewall rules

[Service]
Type=oneshot
ExecStart=/etc/firewall/apply.sh

[Install]
WantedBy=multi-user.target
EOF
systemctl daemon-reload
systemctl enable firewall

# https://docs.oracle.com/en/database/oracle/oracle-database/19/ladbi/checking-resource-limits-for-oracle-software-installation-users.html

echo "root:$(pwgen 32 1)" | chpasswd

# https://www.gnu.org/software/grub/manual/grub/html_node/Serial-terminal.html
# dmesg | grep tty
# cat /proc/tty/driver/serial
if [ ! -f /etc/default/grub.d/loquat.cfg ]
then
    echo 'GRUB_CMDLINE_LINUX="console=tty0 console=ttyS0,115200n8"' >> /etc/default/grub.d/loquat.cfg
    echo 'GRUB_TERMINAL="console serial"' >> /etc/default/grub.d/loquat.cfg
    echo 'GRUB_SERIAL_COMMAND="serial --unit=0 --speed=115200 --word=8 --parity=no --stop=1"' >> /etc/default/grub.d/loquat.cfg
    update-grub

    systemctl enable serial-getty@ttyS0.service
fi

sed -i 's/^ENABLED=.*$/ENABLED="true"/' /etc/default/sysstat
systemctl restart sysstat.service

echo 'done.'
exit 0
