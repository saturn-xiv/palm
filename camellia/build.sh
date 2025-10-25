#!/bin/bash

set -e

export $ERL_HOME=$HOME/.asdf/installs/erlang/28.1.1


"lib/jinterface-1.15/priv/OtpErlang.jar"

mvn clean


echo 'done.'
exit 0
