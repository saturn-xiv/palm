#!/bin/bash

set -e

cp /mnt/authorized_keys $HOME/.ssh/
chmod 644 $HOME/.ssh/authorized_keys

sudo supervisord -c /etc/supervisor/supervisord.conf
