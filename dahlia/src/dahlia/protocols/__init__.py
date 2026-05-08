import base64

from google.protobuf.message import Message as ProtobufMessage


def to_str(m: ProtobufMessage):
    return base64.urlsafe_b64encode(m.SerializeToString())


def from_str(s, m: ProtobufMessage):
    m.ParseFromString(base64.urlsafe_b64decode(s))
