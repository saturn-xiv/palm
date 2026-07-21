#!/bin/bash

set -e

function build_wisteria() {
    echo "building wisteria for $1"
    cargo build --release --target $1
}

# ---------------------------------------------------------

declare -a targets=("x86_64-unknown-linux-gnu" "aarch64-unknown-linux-gnu" "riscv64gc-unknown-linux-gnu")
for t in "${targets[@]}"; do
    build_wisteria $t
done

echo "done."
exit 0
