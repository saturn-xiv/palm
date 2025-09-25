#!/bin/sh

curl -v -X POST \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer <token>" \
    -d '{"lang": "en-US"}' \
    'http://localhost:8180/api/phlox/com.github.saturn_xiv.palm.plugins.monitoring.v1/Site/layout?q=com.github.saturn_xiv.palm.plugins.monitoring.v1.SiteLayoutRequest'
