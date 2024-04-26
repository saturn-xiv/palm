#!/bin/bash

set -e

cd /var/www/bugzilla/

./checksetup.pl
apachectl configtest

# lynx http://localhost:18080/bugzilla
apachectl -D FOREGROUND
