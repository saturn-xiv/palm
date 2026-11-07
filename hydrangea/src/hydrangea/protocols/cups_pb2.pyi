from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class Task(_message.Message):
    __slots__ = ("name", "document", "number_up", "copies", "job_sheet", "media", "orientation", "quality", "sides")
    class JobSheet(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
        __slots__ = ()
        STANDARD: _ClassVar[Task.JobSheet]
        UNCLASSIFIED: _ClassVar[Task.JobSheet]
        CLASSIFIED: _ClassVar[Task.JobSheet]
        CONFIDENTIAL: _ClassVar[Task.JobSheet]
        SECRET: _ClassVar[Task.JobSheet]
        TOP_SECRET: _ClassVar[Task.JobSheet]
    STANDARD: Task.JobSheet
    UNCLASSIFIED: Task.JobSheet
    CLASSIFIED: Task.JobSheet
    CONFIDENTIAL: Task.JobSheet
    SECRET: Task.JobSheet
    TOP_SECRET: Task.JobSheet
    class Media(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
        __slots__ = ()
        A4: _ClassVar[Task.Media]
        A3: _ClassVar[Task.Media]
        LETTER: _ClassVar[Task.Media]
    A4: Task.Media
    A3: Task.Media
    LETTER: Task.Media
    class Orientation(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
        __slots__ = ()
        LABDSCAPE_COUNTER_CLOCKWISE90: _ClassVar[Task.Orientation]
        LABDSCAPE_CLOCKWISE90: _ClassVar[Task.Orientation]
        REVERSE_PORTRAIT: _ClassVar[Task.Orientation]
    LABDSCAPE_COUNTER_CLOCKWISE90: Task.Orientation
    LABDSCAPE_CLOCKWISE90: Task.Orientation
    REVERSE_PORTRAIT: Task.Orientation
    class Quality(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
        __slots__ = ()
        DRAFT: _ClassVar[Task.Quality]
        NORMAL: _ClassVar[Task.Quality]
        BEST: _ClassVar[Task.Quality]
    DRAFT: Task.Quality
    NORMAL: Task.Quality
    BEST: Task.Quality
    class Sides(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
        __slots__ = ()
        ONE: _ClassVar[Task.Sides]
        TWO_LONG: _ClassVar[Task.Sides]
        TWO_SHORT: _ClassVar[Task.Sides]
    ONE: Task.Sides
    TWO_LONG: Task.Sides
    TWO_SHORT: Task.Sides
    NAME_FIELD_NUMBER: _ClassVar[int]
    DOCUMENT_FIELD_NUMBER: _ClassVar[int]
    NUMBER_UP_FIELD_NUMBER: _ClassVar[int]
    COPIES_FIELD_NUMBER: _ClassVar[int]
    JOB_SHEET_FIELD_NUMBER: _ClassVar[int]
    MEDIA_FIELD_NUMBER: _ClassVar[int]
    ORIENTATION_FIELD_NUMBER: _ClassVar[int]
    QUALITY_FIELD_NUMBER: _ClassVar[int]
    SIDES_FIELD_NUMBER: _ClassVar[int]
    name: str
    document: bytes
    number_up: int
    copies: int
    job_sheet: Task.JobSheet
    media: Task.Media
    orientation: Task.Orientation
    quality: Task.Quality
    sides: Task.Sides
    def __init__(self, name: _Optional[str] = ..., document: _Optional[bytes] = ..., number_up: _Optional[int] = ..., copies: _Optional[int] = ..., job_sheet: _Optional[_Union[Task.JobSheet, str]] = ..., media: _Optional[_Union[Task.Media, str]] = ..., orientation: _Optional[_Union[Task.Orientation, str]] = ..., quality: _Optional[_Union[Task.Quality, str]] = ..., sides: _Optional[_Union[Task.Sides, str]] = ...) -> None: ...
