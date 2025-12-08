#!/bin/sh

curl -v -X POST \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer <token>" \
    -d '{"index": 1, "size": 60}' \
    'http://localhost:8180/api/tulip/com.github.saturn_xiv.palm.plugins.blog.v1/Post/index?q=com.github.saturn_xiv.palm.plugins.portal.v1.Page'
