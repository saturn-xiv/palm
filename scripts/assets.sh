#!/bin/sh

set -e

yarn add @material-ui/core@next @emotion/react @emotion/styled @fontsource/roboto @material-ui/icons \
    emoji-mart google-map-react
    js-cookie jwt-decode
    moment moment-timezone
    qrcode.react react-copy-to-clipboard

echo 'done.'
exit 0
