import logging
import socket
from datetime import datetime, UTC
import time
from io import BytesIO
from http.client import HTTPResponse

from fcgi_client import FastCGIClient

from daisy.protocols import lavender_pb2, to_timestamp, to_duration, Duration

logger = logging.getLogger(__name__)


def launch(stub, host):
    logger.info("read php-fpm /status for %s", host)
    hostname = socket.gethostname()

    try:
        it = _status_fetch(host)
        setattr(it, "from", hostname)
        stub.Claw(it)
    except Exception as e:
        logger.exception("fetch php-fpm status")


# sudo -u nobody bash -c "export SCRIPT_NAME=/status; export SCRIPT_FILENAME=/status; export REQUEST_METHOD=GET; cgi-fcgi -bind -connect 127.0.0.1:9000"
def _status_fetch(host):
    now = datetime.now(tz=UTC)
    start_time = time.perf_counter()

    it = lavender_pb2.ClawRequest(
        elapsed=Duration(), created_at=to_timestamp(now))
    try:
        client = FastCGIClient(host)

        res = client.request({
            'SCRIPT_FILENAME': '/status',
            'SCRIPT_NAME': '/status',
            'QUERY_STRING': 'json&full',
            'REQUEST_METHOD': 'GET'
        })

        print(res.decode('utf-8'))

        # logger.debug("$$$1", status_code)
        # logger.debug("$$$2", headers)
        it.http.CopyFrom(lavender_pb2.ClawRequest.Http(
            url=host, status_code=200, content_type="", body=""))
    except Exception as e:
        it.http.CopyFrom(lavender_pb2.ClawRequest.Http(url=host, body=str(e)))

    elapsed = to_duration(time.perf_counter() - start_time)
    it.elapsed.seconds = elapsed.seconds
    it.elapsed.nanos = elapsed.nanos
    return it
