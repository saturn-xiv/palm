import time
import logging
from functools import wraps
from importlib import resources
from pathlib import Path
import uuid

from google.protobuf.timestamp_pb2 import Timestamp
from google.protobuf.duration_pb2 import Duration
import pystache
import pika

logger = logging.getLogger(__name__)


def time_it(func):
    @wraps(func)
    def wrapper(*args, **kwargs):
        start = time.perf_counter()
        result = func(*args, **kwargs)
        elapsed = time.perf_counter() - start
        logger.debug("%s took %s seconds to run",
                     func.__name__, f"{elapsed:.6f}")
        return (result, elapsed)
    return wrapper


def to_duration(dt):
    it = Duration()
    it.seconds = int(dt)
    it.nanos = int((dt - it.seconds) * 1e9)
    return it


def to_timestamp(dt):
    it = Timestamp()
    it.FromDatetime(dt)
    return it


def render(name, data):
    root = Path("templates")
    file = resources.files("hydrangea").joinpath(root/name)
    logger.debug('load casbin model from %s', file)
    tpl = file.read_text(encoding="utf-8")
    return pystache.render(tpl, data)


def protobuf_message_properties():
    return pika.BasicProperties(
        content_type="application/protobuf",
        message_id=str(uuid.uuid4()),
        delivery_mode=pika.DeliveryMode.Persistent,
    )
