#!/bin/bash

set -e
diesel print-schema -e schema_migrations > ../src/schema.rs
exit 0
