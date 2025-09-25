#!/bin/sh

curl -v -X POST \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer <token>" \
    -d '{}' \
    'http://localhost:8180/api/lavender/com.github.saturn_xiv.palm.plugins.portal.v1/Site/timezones?q=google.protobuf.Empty'
