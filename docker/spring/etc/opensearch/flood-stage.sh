#!/bin/sh

curl -v -X PUT "http://localhost:9200/_cluster/settings" \
     -H "Content-Type: application/json" \
     -d '{
  "persistent": {
    "cluster.routing.allocation.disk.watermark.flood_stage": "99%",
    "cluster.routing.allocation.disk.watermark.high": "96%",
    "cluster.routing.allocation.disk.watermark.low": "85%"
  }
}'

