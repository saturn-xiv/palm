#!/bin/bash

set -e

export MIX_ENV=prod
mix deps.get --only prod
mix compile
mix release
exit 0
