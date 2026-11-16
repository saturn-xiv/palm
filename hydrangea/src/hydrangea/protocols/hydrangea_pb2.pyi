from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from collections.abc import Iterable as _Iterable
from typing import ClassVar as _ClassVar, Optional as _Optional

DESCRIPTOR: _descriptor.FileDescriptor

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
