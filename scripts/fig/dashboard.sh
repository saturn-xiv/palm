#!/bin/bash

set -e

# https://vitejs.dev/guide/#scaffolding-your-first-vite-project
# npm create vite@latest fig -- --template react-ts

npm install --save @fortawesome/fontawesome-free \
    famfamfam-flags famfamfam-silk famfamfam-mini \
    js-cookie @types/js-cookie \
    mermaid \
    jwt-decode \
    react-currency-input-field \
    video.js @types/video.js \
    react-copy-to-clipboard @types/react-copy-to-clipboard \
    slate slate-react react-dropzone \
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
    @reduxjs/toolkit react-redux \
    formik yup

# https://blueprintjs.com/docs/#blueprint.quick-start
npm install --save @blueprintjs/core \
    @blueprintjs/icons @blueprintjs/datetime2 @blueprintjs/select @blueprintjs/table

echo 'done.'
exit 0
