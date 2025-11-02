#!/bin/bash

set -e

pacman -Scc --noconfirm
find /tmp -type f -atime +1 -delete

echo 'done.'
exit 0
