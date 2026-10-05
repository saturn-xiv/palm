from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from collections.abc import Iterable as _Iterable, Mapping as _Mapping
from typing import ClassVar as _ClassVar, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class Empty(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class Timestamp(_message.Message):
    __slots__ = ("seconds", "nanos")
    SECONDS_FIELD_NUMBER: _ClassVar[int]
    NANOS_FIELD_NUMBER: _ClassVar[int]
    seconds: int
    nanos: int
    def __init__(self, seconds: _Optional[int] = ..., nanos: _Optional[int] = ...) -> None: ...

class Http(_message.Message):
    __slots__ = ("status_code", "content_type", "response_body")
    STATUS_CODE_FIELD_NUMBER: _ClassVar[int]
    CONTENT_TYPE_FIELD_NUMBER: _ClassVar[int]
    RESPONSE_BODY_FIELD_NUMBER: _ClassVar[int]
    status_code: int
    content_type: str
    response_body: str
    def __init__(self, status_code: _Optional[int] = ..., content_type: _Optional[str] = ..., response_body: _Optional[str] = ...) -> None: ...

class PostgreSql(_message.Message):
    __slots__ = ("version",)
    VERSION_FIELD_NUMBER: _ClassVar[int]
    version: str
    def __init__(self, version: _Optional[str] = ...) -> None: ...

class MySql(_message.Message):
    __slots__ = ("version",)
    VERSION_FIELD_NUMBER: _ClassVar[int]
    version: str
    def __init__(self, version: _Optional[str] = ...) -> None: ...

class Redis(_message.Message):
    __slots__ = ("info", "cluster")
    INFO_FIELD_NUMBER: _ClassVar[int]
    CLUSTER_FIELD_NUMBER: _ClassVar[int]
    info: str
    cluster: str
    def __init__(self, info: _Optional[str] = ..., cluster: _Optional[str] = ...) -> None: ...

class Snmp(_message.Message):
    __slots__ = ("items",)
    class Item(_message.Message):
        __slots__ = ("oid", "i", "s", "d")
        OID_FIELD_NUMBER: _ClassVar[int]
        I_FIELD_NUMBER: _ClassVar[int]
        S_FIELD_NUMBER: _ClassVar[int]
        D_FIELD_NUMBER: _ClassVar[int]
        oid: str
        i: int
        s: str
        d: float
        def __init__(self, oid: _Optional[str] = ..., i: _Optional[int] = ..., s: _Optional[str] = ..., d: _Optional[float] = ...) -> None: ...
    ITEMS_FIELD_NUMBER: _ClassVar[int]
    items: _containers.RepeatedCompositeFieldContainer[Snmp.Item]
    def __init__(self, items: _Optional[_Iterable[_Union[Snmp.Item, _Mapping]]] = ...) -> None: ...

class ReportRequest(_message.Message):
    __slots__ = ("items",)
    class Item(_message.Message):
        __slots__ = ("timestamp", "http", "postgresql", "mysql", "redis", "snmp")
        TIMESTAMP_FIELD_NUMBER: _ClassVar[int]
        HTTP_FIELD_NUMBER: _ClassVar[int]
        POSTGRESQL_FIELD_NUMBER: _ClassVar[int]
        MYSQL_FIELD_NUMBER: _ClassVar[int]
        REDIS_FIELD_NUMBER: _ClassVar[int]
        SNMP_FIELD_NUMBER: _ClassVar[int]
        timestamp: Timestamp
        http: Http
        postgresql: PostgreSql
        mysql: MySql
        redis: Redis
        snmp: Snmp
        def __init__(self, timestamp: _Optional[_Union[Timestamp, _Mapping]] = ..., http: _Optional[_Union[Http, _Mapping]] = ..., postgresql: _Optional[_Union[PostgreSql, _Mapping]] = ..., mysql: _Optional[_Union[MySql, _Mapping]] = ..., redis: _Optional[_Union[Redis, _Mapping]] = ..., snmp: _Optional[_Union[Snmp, _Mapping]] = ...) -> None: ...
    ITEMS_FIELD_NUMBER: _ClassVar[int]
    items: _containers.RepeatedCompositeFieldContainer[ReportRequest.Item]
    def __init__(self, items: _Optional[_Iterable[_Union[ReportRequest.Item, _Mapping]]] = ...) -> None: ...
