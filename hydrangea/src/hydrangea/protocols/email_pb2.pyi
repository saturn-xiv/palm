from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from collections.abc import Iterable as _Iterable, Mapping as _Mapping
from typing import ClassVar as _ClassVar, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class Task(_message.Message):
    __slots__ = ("to", "reply_to", "cc", "bcc", "subject", "body", "attachments")
    class Address(_message.Message):
        __slots__ = ("name", "email")
        NAME_FIELD_NUMBER: _ClassVar[int]
        EMAIL_FIELD_NUMBER: _ClassVar[int]
        name: str
        email: str
        def __init__(self, name: _Optional[str] = ..., email: _Optional[str] = ...) -> None: ...
    class Body(_message.Message):
        __slots__ = ("html", "content")
        HTML_FIELD_NUMBER: _ClassVar[int]
        CONTENT_FIELD_NUMBER: _ClassVar[int]
        html: bool
        content: str
        def __init__(self, html: _Optional[bool] = ..., content: _Optional[str] = ...) -> None: ...
    class Attachment(_message.Message):
        __slots__ = ("inline_id", "name", "content_type", "content")
        INLINE_ID_FIELD_NUMBER: _ClassVar[int]
        NAME_FIELD_NUMBER: _ClassVar[int]
        CONTENT_TYPE_FIELD_NUMBER: _ClassVar[int]
        CONTENT_FIELD_NUMBER: _ClassVar[int]
        inline_id: str
        name: str
        content_type: str
        content: bytes
        def __init__(self, inline_id: _Optional[str] = ..., name: _Optional[str] = ..., content_type: _Optional[str] = ..., content: _Optional[bytes] = ...) -> None: ...
    TO_FIELD_NUMBER: _ClassVar[int]
    REPLY_TO_FIELD_NUMBER: _ClassVar[int]
    CC_FIELD_NUMBER: _ClassVar[int]
    BCC_FIELD_NUMBER: _ClassVar[int]
    SUBJECT_FIELD_NUMBER: _ClassVar[int]
    BODY_FIELD_NUMBER: _ClassVar[int]
    ATTACHMENTS_FIELD_NUMBER: _ClassVar[int]
    to: Task.Address
    reply_to: Task.Address
    cc: _containers.RepeatedCompositeFieldContainer[Task.Address]
    bcc: _containers.RepeatedCompositeFieldContainer[Task.Address]
    subject: str
    body: Task.Body
    attachments: _containers.RepeatedCompositeFieldContainer[Task.Attachment]
    def __init__(self, to: _Optional[_Union[Task.Address, _Mapping]] = ..., reply_to: _Optional[_Union[Task.Address, _Mapping]] = ..., cc: _Optional[_Iterable[_Union[Task.Address, _Mapping]]] = ..., bcc: _Optional[_Iterable[_Union[Task.Address, _Mapping]]] = ..., subject: _Optional[str] = ..., body: _Optional[_Union[Task.Body, _Mapping]] = ..., attachments: _Optional[_Iterable[_Union[Task.Attachment, _Mapping]]] = ...) -> None: ...
