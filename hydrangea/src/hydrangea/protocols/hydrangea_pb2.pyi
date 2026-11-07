import datetime

from google.protobuf import duration_pb2 as _duration_pb2
from google.protobuf import empty_pb2 as _empty_pb2
from google.protobuf import timestamp_pb2 as _timestamp_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from collections.abc import Iterable as _Iterable, Mapping as _Mapping
from typing import ClassVar as _ClassVar, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class StashJobRequest(_message.Message):
    __slots__ = ("sn", "status_code", "stdout", "stderr", "launched_at", "elapsed")
    SN_FIELD_NUMBER: _ClassVar[int]
    STATUS_CODE_FIELD_NUMBER: _ClassVar[int]
    STDOUT_FIELD_NUMBER: _ClassVar[int]
    STDERR_FIELD_NUMBER: _ClassVar[int]
    LAUNCHED_AT_FIELD_NUMBER: _ClassVar[int]
    ELAPSED_FIELD_NUMBER: _ClassVar[int]
    sn: str
    status_code: int
    stdout: str
    stderr: str
    launched_at: _timestamp_pb2.Timestamp
    elapsed: _duration_pb2.Duration
    def __init__(self, sn: _Optional[str] = ..., status_code: _Optional[int] = ..., stdout: _Optional[str] = ..., stderr: _Optional[str] = ..., launched_at: _Optional[_Union[datetime.datetime, _timestamp_pb2.Timestamp, _Mapping]] = ..., elapsed: _Optional[_Union[datetime.timedelta, _duration_pb2.Duration, _Mapping]] = ...) -> None: ...

class Task(_message.Message):
    __slots__ = ("sn", "name", "summary", "version", "script", "args")
    SN_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    SUMMARY_FIELD_NUMBER: _ClassVar[int]
    VERSION_FIELD_NUMBER: _ClassVar[int]
    SCRIPT_FIELD_NUMBER: _ClassVar[int]
    ARGS_FIELD_NUMBER: _ClassVar[int]
    sn: str
    name: str
    summary: str
    version: str
    script: str
    args: _containers.RepeatedScalarFieldContainer[str]
    def __init__(self, sn: _Optional[str] = ..., name: _Optional[str] = ..., summary: _Optional[str] = ..., version: _Optional[str] = ..., script: _Optional[str] = ..., args: _Optional[_Iterable[str]] = ...) -> None: ...
