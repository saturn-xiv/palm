#!/bin/bash

set -e

# https://vite.dev/guide/#scaffolding-your-first-vite-project
npm install --save \
    bulma \
    react-router react-redux react-intl formik \
    jsonwebtoken js-cookie @types/js-cookie

exit 0
