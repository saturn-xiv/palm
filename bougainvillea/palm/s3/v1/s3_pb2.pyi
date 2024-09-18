from google.protobuf import duration_pb2 as _duration_pb2
from google.protobuf import empty_pb2 as _empty_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class Bucket(_message.Message):
    __slots__ = ("name", "public", "expiration_days")
    NAME_FIELD_NUMBER: _ClassVar[int]
    PUBLIC_FIELD_NUMBER: _ClassVar[int]
    EXPIRATION_DAYS_FIELD_NUMBER: _ClassVar[int]
    name: str
    public: bool
    expiration_days: int
    def __init__(self, name: _Optional[str] = ..., public: bool = ..., expiration_days: _Optional[int] = ...) -> None: ...

class CreateBucketRequest(_message.Message):
    __slots__ = ("name", "public", "expiration_days")
    NAME_FIELD_NUMBER: _ClassVar[int]
    PUBLIC_FIELD_NUMBER: _ClassVar[int]
    EXPIRATION_DAYS_FIELD_NUMBER: _ClassVar[int]
    name: str
    public: bool
    expiration_days: int
    def __init__(self, name: _Optional[str] = ..., public: bool = ..., expiration_days: _Optional[int] = ...) -> None: ...

class ListBucketResponse(_message.Message):
    __slots__ = ("items",)
    class Item(_message.Message):
        __slots__ = ("name",)
        NAME_FIELD_NUMBER: _ClassVar[int]
        name: str
        def __init__(self, name: _Optional[str] = ...) -> None: ...
    ITEMS_FIELD_NUMBER: _ClassVar[int]
    items: _containers.RepeatedCompositeFieldContainer[ListBucketResponse.Item]
    def __init__(self, items: _Optional[_Iterable[_Union[ListBucketResponse.Item, _Mapping]]] = ...) -> None: ...

class PresignedPutObjectRequest(_message.Message):
    __slots__ = ("bucket", "title", "ttl")
    BUCKET_FIELD_NUMBER: _ClassVar[int]
    TITLE_FIELD_NUMBER: _ClassVar[int]
    TTL_FIELD_NUMBER: _ClassVar[int]
    bucket: str
    title: str
    ttl: _duration_pb2.Duration
    def __init__(self, bucket: _Optional[str] = ..., title: _Optional[str] = ..., ttl: _Optional[_Union[_duration_pb2.Duration, _Mapping]] = ...) -> None: ...

class PresignedPutObjectResponse(_message.Message):
    __slots__ = ("object", "url")
    OBJECT_FIELD_NUMBER: _ClassVar[int]
    URL_FIELD_NUMBER: _ClassVar[int]
    object: str
    url: str
    def __init__(self, object: _Optional[str] = ..., url: _Optional[str] = ...) -> None: ...

class UrlResponse(_message.Message):
    __slots__ = ("url",)
    URL_FIELD_NUMBER: _ClassVar[int]
    url: str
    def __init__(self, url: _Optional[str] = ...) -> None: ...

class ObjectPresignedUrlRequest(_message.Message):
    __slots__ = ("bucket", "object", "title", "content_type", "ttl")
    BUCKET_FIELD_NUMBER: _ClassVar[int]
    OBJECT_FIELD_NUMBER: _ClassVar[int]
    TITLE_FIELD_NUMBER: _ClassVar[int]
    CONTENT_TYPE_FIELD_NUMBER: _ClassVar[int]
    TTL_FIELD_NUMBER: _ClassVar[int]
    bucket: str
    object: str
    title: str
    content_type: str
    ttl: _duration_pb2.Duration
    def __init__(self, bucket: _Optional[str] = ..., object: _Optional[str] = ..., title: _Optional[str] = ..., content_type: _Optional[str] = ..., ttl: _Optional[_Union[_duration_pb2.Duration, _Mapping]] = ...) -> None: ...

class ObjectPermanentUrlRequest(_message.Message):
    __slots__ = ("bucket", "object", "title", "content_type")
    BUCKET_FIELD_NUMBER: _ClassVar[int]
    OBJECT_FIELD_NUMBER: _ClassVar[int]
    TITLE_FIELD_NUMBER: _ClassVar[int]
    CONTENT_TYPE_FIELD_NUMBER: _ClassVar[int]
    bucket: str
    object: str
    title: str
    content_type: str
    def __init__(self, bucket: _Optional[str] = ..., object: _Optional[str] = ..., title: _Optional[str] = ..., content_type: _Optional[str] = ...) -> None: ...

class RemoveObjectRequest(_message.Message):
    __slots__ = ("bucket", "object")
    BUCKET_FIELD_NUMBER: _ClassVar[int]
    OBJECT_FIELD_NUMBER: _ClassVar[int]
    bucket: str
    object: str
    def __init__(self, bucket: _Optional[str] = ..., object: _Optional[str] = ...) -> None: ...

class RemoveBucketRequest(_message.Message):
    __slots__ = ("bucket",)
    BUCKET_FIELD_NUMBER: _ClassVar[int]
    bucket: str
    def __init__(self, bucket: _Optional[str] = ...) -> None: ...
