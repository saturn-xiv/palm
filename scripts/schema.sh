#!/bin/bash

set -e

export WORK_DIR=$PWD

flatc -o portal/src/protocols --filename-suffix "" --rust protocols/email.fbs
flatc -o portal/src/protocols --filename-suffix "" --rust protocols/tex.fbs

export DATABASE_URL="postgres://postgres@127.0.0.1:5432/wisteria_dev?sslmode=disable"
diesel print-schema -o schema_migrations > $WORK_DIR/portal/src/schema.rs

cargo fmt

echo 'done.'
exit 0
