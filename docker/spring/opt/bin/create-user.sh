#!/bin/bash

set -e

if [ "$#" -ne 1 ]; then
    echo "USAGE: $0 USER"
    exit 1
fi

if [ -d /mnt/$1 ]
then
    echo "User $1 already exists."
    exit 1
fi

useradd -s /bin/zsh -m -d /mnt/$1 $1
passwd -l $1
sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)" $1

su - $1 <<EOF
mkdir -pv $HOME/.ssh
chmod 700 $HOME/.ssh
touch $HOME/.ssh/authorized_keys
EOF

echo "Done(please append $1 into AllowUsers session of /etc/ssh/sshd_config)."
exit 0
