#!/bin/bash

set -e

npm install --save \
    moment moment-timezone \
    js-cookie @types/js-cookie \
    jwt-decode \
    react-copy-to-clipboard @types/react-copy-to-clipboard \
    react-quill react-dropzone \
    react-intl \
    react-router-dom@latest \
    react-helmet-async \
    @reduxjs/toolkit react-redux

npm install --save \
    react-bootstrap bootstrap

echo 'done.'
exit 0
