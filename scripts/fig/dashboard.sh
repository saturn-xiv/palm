#!/bin/bash

set -e

# https://vitejs.dev/guide/#scaffolding-your-first-vite-project
# npm create vite@latest fig -- --template react-ts

npm install --save filesize timezones-list \
    marked \
    diff @types/diff \
    lodash @types/lodash \
    @fortawesome/fontawesome-free \
    famfamfam-flags famfamfam-silk famfamfam-mini \
    js-cookie @types/js-cookie \
    mermaid \
    jwt-decode \
    dinero.js@alpha @dinero.js/currencies@alpha react-currency-input-field \
    video.js @types/video.js \
    react-copy-to-clipboard @types/react-copy-to-clipboard \
    slate slate-react react-dropzone \
    google-map-react qrcode.react \
    react-markdown @uiw/react-md-editor \
    react-color @types/react-color \
    react-pdf \
    react-syntax-highlighter \
    emoji-mart react-sparklines react-highlight-words \
    react-number-format react-image-crop \
    react-player \
    react-draggable \
    react-big-calendar @types/react-big-calendar \
    react-intl \
    react-router-dom@latest \
    @reduxjs/toolkit react-redux

# https://mui.com/material-ui/getting-started/installation/
npm install --save @mui/material @emotion/react @emotion/styled \
    @fontsource/roboto @mui/icons-material @mui/x-charts

# npm install --save grpc-web google-protobuf @types/google-protobuf

# npm install --save \
#     antd @ant-design/pro-components @ant-design/charts \
#     @emotion/css

echo 'done.'
exit 0
