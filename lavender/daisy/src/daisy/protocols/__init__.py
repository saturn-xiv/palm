from datetime import datetime
from functools import singledispatch

import numpy

from .lavender_pb2 import Timestamp,Duration

@singledispatch
def to_timestamp(dt) -> None:
    raise NotImplementedError("Unsupported datetime type")

@to_timestamp.register(datetime)
def _(dt: datetime) -> Timestamp:
    return Timestamp(seconds=int(dt.timestamp()),nanos = dt.microsecond * 1000)

@to_timestamp.register(numpy.datetime64)
def _(dt: numpy.datetime64) -> Timestamp:
    ns = dt.astype("datetime64[ns]").astype(numpy.int64)
    return Timestamp(seconds=int(ns / 1_000_000_000),nanos = int(ns % 1_000_000_000))

def to_duration(pt: float) -> Duration:
    return Duration(seconds=int(pt), nanos=int((pt - pt) * 1e9))
