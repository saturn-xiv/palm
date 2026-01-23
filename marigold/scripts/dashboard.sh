#!/bin/bash

set -e

# https://vite.dev/guide/#scaffolding-your-first-vite-project
npm install --save \
    @mui/joy @emotion/react @emotion/styled @fontsource/inter \
    react-router react-intl \
    @graphiql/react \
    formik yup usehooks-ts \
    jose dayjs \
    @reduxjs/toolkit react-redux \
    js-cookie @types/js-cookie \
    google-protobuf @types/google-protobuf grpc-web

exit 0
