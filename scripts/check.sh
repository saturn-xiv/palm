#!/bin/bash

set -e

cargo fmt
cargo clippy
cargo build

echo "done."
exit 0
