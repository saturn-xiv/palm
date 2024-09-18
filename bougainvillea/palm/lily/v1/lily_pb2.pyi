from google.protobuf import duration_pb2 as _duration_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class Style(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = ()
    APA_7: _ClassVar[Style]
    MLA_8: _ClassVar[Style]
    CMOS: _ClassVar[Style]
APA_7: Style
MLA_8: Style
CMOS: Style

class Book(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class Slideshow(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class Article(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class Section(_message.Message):
    __slots__ = ("items",)
    ITEMS_FIELD_NUMBER: _ClassVar[int]
    items: _containers.RepeatedCompositeFieldContainer[Section]
    def __init__(self, items: _Optional[_Iterable[_Union[Section, _Mapping]]] = ...) -> None: ...

class TeXLiveRequest(_message.Message):
    __slots__ = ("name", "style", "format", "ttl", "book", "article", "slideshow", "entry", "attachments")
    class AttachmentsEntry(_message.Message):
        __slots__ = ("key", "value")
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: bytes
        def __init__(self, key: _Optional[str] = ..., value: _Optional[bytes] = ...) -> None: ...
    NAME_FIELD_NUMBER: _ClassVar[int]
    STYLE_FIELD_NUMBER: _ClassVar[int]
    FORMAT_FIELD_NUMBER: _ClassVar[int]
    TTL_FIELD_NUMBER: _ClassVar[int]
    BOOK_FIELD_NUMBER: _ClassVar[int]
    ARTICLE_FIELD_NUMBER: _ClassVar[int]
    SLIDESHOW_FIELD_NUMBER: _ClassVar[int]
    ENTRY_FIELD_NUMBER: _ClassVar[int]
    ATTACHMENTS_FIELD_NUMBER: _ClassVar[int]
    name: str
    style: Style
    format: TeXLiveTask.Output.Format
    ttl: _duration_pb2.Duration
    book: Book
    article: Article
    slideshow: Slideshow
    entry: bytes
    attachments: _containers.ScalarMap[str, bytes]
    def __init__(self, name: _Optional[str] = ..., style: _Optional[_Union[Style, str]] = ..., format: _Optional[_Union[TeXLiveTask.Output.Format, str]] = ..., ttl: _Optional[_Union[_duration_pb2.Duration, _Mapping]] = ..., book: _Optional[_Union[Book, _Mapping]] = ..., article: _Optional[_Union[Article, _Mapping]] = ..., slideshow: _Optional[_Union[Slideshow, _Mapping]] = ..., entry: _Optional[bytes] = ..., attachments: _Optional[_Mapping[str, bytes]] = ...) -> None: ...

class TeXLiveResponse(_message.Message):
    __slots__ = ("bucket", "object")
    BUCKET_FIELD_NUMBER: _ClassVar[int]
    OBJECT_FIELD_NUMBER: _ClassVar[int]
    bucket: str
    object: str
    def __init__(self, bucket: _Optional[str] = ..., object: _Optional[str] = ...) -> None: ...

class TeXLiveTask(_message.Message):
    __slots__ = ("entry", "attachments", "output")
    class AttachmentsEntry(_message.Message):
        __slots__ = ("key", "value")
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: bytes
        def __init__(self, key: _Optional[str] = ..., value: _Optional[bytes] = ...) -> None: ...
    class Output(_message.Message):
        __slots__ = ("format", "bucket", "object")
        class Format(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
            __slots__ = ()
            Pdf: _ClassVar[TeXLiveTask.Output.Format]
        Pdf: TeXLiveTask.Output.Format
        FORMAT_FIELD_NUMBER: _ClassVar[int]
        BUCKET_FIELD_NUMBER: _ClassVar[int]
        OBJECT_FIELD_NUMBER: _ClassVar[int]
        format: TeXLiveTask.Output.Format
        bucket: str
        object: str
        def __init__(self, format: _Optional[_Union[TeXLiveTask.Output.Format, str]] = ..., bucket: _Optional[str] = ..., object: _Optional[str] = ...) -> None: ...
    ENTRY_FIELD_NUMBER: _ClassVar[int]
    ATTACHMENTS_FIELD_NUMBER: _ClassVar[int]
    OUTPUT_FIELD_NUMBER: _ClassVar[int]
    entry: bytes
    attachments: _containers.ScalarMap[str, bytes]
    output: TeXLiveTask.Output
    def __init__(self, entry: _Optional[bytes] = ..., attachments: _Optional[_Mapping[str, bytes]] = ..., output: _Optional[_Union[TeXLiveTask.Output, _Mapping]] = ...) -> None: ...

class PandocTask(_message.Message):
    __slots__ = ("input", "output")
    class Format(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
        __slots__ = ()
        Plain: _ClassVar[PandocTask.Format]
        Markdown: _ClassVar[PandocTask.Format]
        Pdf: _ClassVar[PandocTask.Format]
        Docx: _ClassVar[PandocTask.Format]
    Plain: PandocTask.Format
    Markdown: PandocTask.Format
    Pdf: PandocTask.Format
    Docx: PandocTask.Format
    class Input(_message.Message):
        __slots__ = ("format", "payload")
        FORMAT_FIELD_NUMBER: _ClassVar[int]
        PAYLOAD_FIELD_NUMBER: _ClassVar[int]
        format: PandocTask.Format
        payload: bytes
        def __init__(self, format: _Optional[_Union[PandocTask.Format, str]] = ..., payload: _Optional[bytes] = ...) -> None: ...
    class Output(_message.Message):
        __slots__ = ("format", "bucket", "object")
        FORMAT_FIELD_NUMBER: _ClassVar[int]
        BUCKET_FIELD_NUMBER: _ClassVar[int]
        OBJECT_FIELD_NUMBER: _ClassVar[int]
        format: PandocTask.Format
        bucket: str
        object: str
        def __init__(self, format: _Optional[_Union[PandocTask.Format, str]] = ..., bucket: _Optional[str] = ..., object: _Optional[str] = ...) -> None: ...
    INPUT_FIELD_NUMBER: _ClassVar[int]
    OUTPUT_FIELD_NUMBER: _ClassVar[int]
    input: PandocTask.Input
    output: PandocTask.Output
    def __init__(self, input: _Optional[_Union[PandocTask.Input, _Mapping]] = ..., output: _Optional[_Union[PandocTask.Output, _Mapping]] = ...) -> None: ...
