#!/bin/bash

set -e

# https://vitejs.dev/guide/#scaffolding-your-first-vite-project
# npm create vite@latest fig -- --template react-ts

npm install --save @fortawesome/fontawesome-free \
    famfamfam-flags famfamfam-silk famfamfam-mini \
    js-cookie @types/js-cookie \
    filesize \
    mermaid \
    jwt-decode \
    react-currency-input-field \
    video.js @types/video.js \
    react-copy-to-clipboard @types/react-copy-to-clipboard \
    slate slate-react \
    google-map-react qrcode.react \
    @uiw/react-md-editor \
    react-color @types/react-color \
    react-pdf \
    react-syntax-highlighter \
    emoji-mart react-sparklines react-highlight-words \
    react-player \
    react-draggable \
    react-big-calendar @types/react-big-calendar \
    react-intl \
    react-router-dom@latest \
    @reduxjs/toolkit react-redux

# https://procomponents.ant.design/en-US/docs
# https://ant-design-charts.antgroup.com/en/manual/getting-started
npm install --save antd @ant-design/pro-components @ant-design/charts

echo 'done.'
exit 0
