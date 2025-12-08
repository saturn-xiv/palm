#!/bin/sh

curl -v -X POST \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer <token>" \
    -d '{}' \
    'http://localhost:8180/api/daisy/com.github.saturn_xiv.palm.plugins.portal.v1/Site/timezones?q=com.google.protobuf.Empty'
