#!/bin/sh

curl -v -X POST \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer <token>" \
    -d '{}' \
    'http://localhost:8080/lavender/com.github.saturn_xiv.palm.plugins.portal.v1/Site/Timezones?q=google.protobuf.Empty&p=com.github.saturn_xiv.palm.plugins.portal.v1.SiteTimezonesResponse'
