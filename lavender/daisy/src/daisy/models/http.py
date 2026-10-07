import logging
import socket
from datetime import datetime, UTC
import time

import requests

from daisy.protocols import lavender_pb2, to_timestamp, to_duration, Duration

logger = logging.getLogger(__name__)

INDEX_NAME = "hyacinth.palm.lavender.v1.internal_do_not_use_lavender.http_request.item"


def launch(stub, url):
    logger.info("load %s", url)
    hostname = socket.gethostname()

    try:
        it = _http_fetch(url)
        stub.Http(lavender_pb2.HttpRequest(host=hostname, items=[it]))
    except Exception as e:
        logger.exception("fetch http")


def _http_fetch(url):
    now = datetime.now(tz=UTC)
    start_time = time.perf_counter()

    it = lavender_pb2.HttpRequest.Item(
        url=url, elapsed=Duration(), created_at=to_timestamp(now))
    try:
        res = requests.get(url, timeout=5)
        it.status_code = res.status_code
        it.content_type = res.headers['Content-Type']
        it.body = res.text
    except Exception as e:
        it.body = str(e)

    elapsed = to_duration(time.perf_counter() - start_time)
    it.elapsed.seconds = elapsed.seconds
    it.elapsed.nanos = elapsed.nanos
    return it
