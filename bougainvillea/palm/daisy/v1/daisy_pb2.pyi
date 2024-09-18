from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class SmsTask(_message.Message):
    __slots__ = ("to", "body", "callback_uri")
    TO_FIELD_NUMBER: _ClassVar[int]
    BODY_FIELD_NUMBER: _ClassVar[int]
    CALLBACK_URI_FIELD_NUMBER: _ClassVar[int]
    to: _containers.RepeatedScalarFieldContainer[str]
    body: str
    callback_uri: str
    def __init__(self, to: _Optional[_Iterable[str]] = ..., body: _Optional[str] = ..., callback_uri: _Optional[str] = ...) -> None: ...

class EmailTask(_message.Message):
    __slots__ = ("subject", "body", "to", "cc", "bcc", "attachments")
    class Address(_message.Message):
        __slots__ = ("name", "email")
        NAME_FIELD_NUMBER: _ClassVar[int]
        EMAIL_FIELD_NUMBER: _ClassVar[int]
        name: str
        email: str
        def __init__(self, name: _Optional[str] = ..., email: _Optional[str] = ...) -> None: ...
    class Body(_message.Message):
        __slots__ = ("text", "html")
        TEXT_FIELD_NUMBER: _ClassVar[int]
        HTML_FIELD_NUMBER: _ClassVar[int]
        text: str
        html: bool
        def __init__(self, text: _Optional[str] = ..., html: bool = ...) -> None: ...
    class Attachment(_message.Message):
        __slots__ = ("title", "content_type", "inline", "body")
        TITLE_FIELD_NUMBER: _ClassVar[int]
        CONTENT_TYPE_FIELD_NUMBER: _ClassVar[int]
        INLINE_FIELD_NUMBER: _ClassVar[int]
        BODY_FIELD_NUMBER: _ClassVar[int]
        title: str
        content_type: str
        inline: bool
        body: bytes
        def __init__(self, title: _Optional[str] = ..., content_type: _Optional[str] = ..., inline: bool = ..., body: _Optional[bytes] = ...) -> None: ...
    SUBJECT_FIELD_NUMBER: _ClassVar[int]
    BODY_FIELD_NUMBER: _ClassVar[int]
    TO_FIELD_NUMBER: _ClassVar[int]
    CC_FIELD_NUMBER: _ClassVar[int]
    BCC_FIELD_NUMBER: _ClassVar[int]
    ATTACHMENTS_FIELD_NUMBER: _ClassVar[int]
    subject: str
    body: EmailTask.Body
    to: EmailTask.Address
    cc: _containers.RepeatedCompositeFieldContainer[EmailTask.Address]
    bcc: _containers.RepeatedCompositeFieldContainer[EmailTask.Address]
    attachments: _containers.RepeatedCompositeFieldContainer[EmailTask.Attachment]
    def __init__(self, subject: _Optional[str] = ..., body: _Optional[_Union[EmailTask.Body, _Mapping]] = ..., to: _Optional[_Union[EmailTask.Address, _Mapping]] = ..., cc: _Optional[_Iterable[_Union[EmailTask.Address, _Mapping]]] = ..., bcc: _Optional[_Iterable[_Union[EmailTask.Address, _Mapping]]] = ..., attachments: _Optional[_Iterable[_Union[EmailTask.Attachment, _Mapping]]] = ...) -> None: ...
