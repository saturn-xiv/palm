from google.protobuf.internal import containers as _containers
from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from collections.abc import Mapping as _Mapping
from typing import ClassVar as _ClassVar, Optional as _Optional

DESCRIPTOR: _descriptor.FileDescriptor

class Task(_message.Message):
    __slots__ = ("entry", "files", "output")
    class Class(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
        __slots__ = ()
        ARTICLE: _ClassVar[Task.Class]
        REPORT: _ClassVar[Task.Class]
        BOOK: _ClassVar[Task.Class]
        LETTER: _ClassVar[Task.Class]
        BEAMER: _ClassVar[Task.Class]
    ARTICLE: Task.Class
    REPORT: Task.Class
    BOOK: Task.Class
    LETTER: Task.Class
    BEAMER: Task.Class
    class FilesEntry(_message.Message):
        __slots__ = ("key", "value")
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: bytes
        def __init__(self, key: _Optional[str] = ..., value: _Optional[bytes] = ...) -> None: ...
    ENTRY_FIELD_NUMBER: _ClassVar[int]
    CLASS_FIELD_NUMBER: _ClassVar[int]
    FILES_FIELD_NUMBER: _ClassVar[int]
    OUTPUT_FIELD_NUMBER: _ClassVar[int]
    entry: str
    files: _containers.ScalarMap[str, bytes]
    output: str
    def __init__(self, entry: _Optional[str] = ..., files: _Optional[_Mapping[str, bytes]] = ..., output: _Optional[str] = ..., **kwargs) -> None: ...
