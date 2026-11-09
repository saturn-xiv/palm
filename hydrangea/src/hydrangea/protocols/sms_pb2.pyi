from google.protobuf.internal import containers as _containers
from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from collections.abc import Iterable as _Iterable
from typing import ClassVar as _ClassVar, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class Task(_message.Message):
    __slots__ = ("to", "body", "status_callback", "by")
    class By(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
        __slots__ = ()
        TWILIO: _ClassVar[Task.By]
    TWILIO: Task.By
    FROM_FIELD_NUMBER: _ClassVar[int]
    TO_FIELD_NUMBER: _ClassVar[int]
    BODY_FIELD_NUMBER: _ClassVar[int]
    STATUS_CALLBACK_FIELD_NUMBER: _ClassVar[int]
    BY_FIELD_NUMBER: _ClassVar[int]
    to: _containers.RepeatedScalarFieldContainer[str]
    body: str
    status_callback: str
    by: Task.By
    def __init__(self, to: _Optional[_Iterable[str]] = ..., body: _Optional[str] = ..., status_callback: _Optional[str] = ..., by: _Optional[_Union[Task.By, str]] = ..., **kwargs) -> None: ...
