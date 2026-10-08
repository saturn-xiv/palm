import logging
import socket
from datetime import datetime, UTC
import time

import requests

from daisy.protocols import lavender_pb2, to_timestamp, to_duration, Duration

logger = logging.getLogger(__name__)

INDEX_NAME = "lavender.graphql.monitoring.http.item"


def launch(stub, url):
    logger.info("load %s", url)
    hostname = socket.gethostname()

    try:
        it = _http_fetch(url)
        setattr(it, "from", hostname)
        stub.Claw(it)
    except Exception as e:
        logger.exception("fetch http")


def _http_fetch(url):
    now = datetime.now(tz=UTC)
    start_time = time.perf_counter()

    it = lavender_pb2.ClawRequest(
        elapsed=Duration(), created_at=to_timestamp(now))
    try:
        res = requests.get(url, timeout=5)
        it.http.CopyFrom(lavender_pb2.ClawRequest.Http(
            url=url, status_code=res.status_code, content_type=res.headers['Content-Type'], body=res.text))
    except Exception as e:
        it.http.CopyFrom(lavender_pb2.ClawRequest.Http(url=url, body=str(e)))

    elapsed = to_duration(time.perf_counter() - start_time)
    it.elapsed.seconds = elapsed.seconds
    it.elapsed.nanos = elapsed.nanos
    return it
