#!/bin/bash

set -e

npm install --save \
    bootstrap bulma @picocss/pico @material/web \
    foundation-sites \
    @tabler/core @tabler/icons @tabler/icons-webfont \
    @fortawesome/fontawesome-free dayjs \
    marked dompurify jsdom

exit 0
