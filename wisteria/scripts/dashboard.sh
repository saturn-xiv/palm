#!/bin/bash

set -e

# https://vite.dev/guide/#scaffolding-your-first-vite-project
# https://mui.com/material-ui/getting-started/installation/
npm install --save \
    @mui/material @emotion/react @emotion/styled @fontsource/inter @fontsource/roboto @mui/icons-material \
    react-router react-intl \
    remark-gfm react-markdown @mdxeditor/editor \
    slate slate-history slate-react \
    @graphiql/react \
    formik yup usehooks-ts \
    jose dayjs video.js \
    @reduxjs/toolkit react-redux \
    js-cookie @types/js-cookie jwt-decode \
    flatbuffers google-protobuf @types/google-protobuf grpc-web

exit 0
