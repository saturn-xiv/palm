#!/bin/bash

set -e

# https://fontsource.org/docs/getting-started/install
# https://fontsource.org/docs/getting-started/variable

npm install marked moment moment-timezone d3 mermaid qrcodejs epubjs \
    bootstrap @popperjs/core \
    mdb-ui-kit@latest \
    swagger-ui-dist \
    bulma \
    tailwindcss@latest \
    @tabler/core \
    material-components-web material-design-icons \
    @fontsource/roboto @fontsource-variable/montserrat \
    @fortawesome/fontawesome-free \
    famfamfam-flags famfamfam-silk famfamfam-mini \
    --save

echo 'done.'
exit 0
