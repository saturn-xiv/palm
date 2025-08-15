#!/bin/sh

curl -v -X POST \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer <token>" \
    -d '{"lang": "en-US"}' \
    'http://localhost:8080/phlox/com.github.saturn_xiv.palm.plugins.monitoring.v1/Site/Layout?q=com.github.saturn_xiv.palm.plugins.monitoring.v1.SiteLayoutRequest&p=com.github.saturn_xiv.palm.plugins.monitoring.v1.SiteLayoutResponse'
