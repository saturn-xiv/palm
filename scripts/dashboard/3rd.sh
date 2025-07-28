#!/bin/sh

set -e

# npm create vite@latest dashboard -- --template react-ts

npm install --save \
    @mui/material @emotion/react @emotion/styled @mui/icons-material @fontsource/roboto \
    react-intl react-router @reduxjs/toolkit react-redux
     

echo 'done.'
