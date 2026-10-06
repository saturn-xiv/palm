from .lavender_pb2 import Timestamp

def to_timestamp(dt):
    return Timestamp(seconds=int(dt.timestamp()),nanos = dt.microsecond * 1000)
