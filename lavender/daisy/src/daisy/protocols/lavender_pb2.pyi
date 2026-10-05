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

class Duration(_message.Message):
    __slots__ = ("seconds", "nanos")
    SECONDS_FIELD_NUMBER: _ClassVar[int]
    NANOS_FIELD_NUMBER: _ClassVar[int]
    seconds: int
    nanos: int
    def __init__(self, seconds: _Optional[int] = ..., nanos: _Optional[int] = ...) -> None: ...

class SystemdRequest(_message.Message):
    __slots__ = ("items",)
    class Item(_message.Message):
        __slots__ = ("host", "name", "pid", "message", "created_at")
        HOST_FIELD_NUMBER: _ClassVar[int]
        NAME_FIELD_NUMBER: _ClassVar[int]
        PID_FIELD_NUMBER: _ClassVar[int]
        MESSAGE_FIELD_NUMBER: _ClassVar[int]
        CREATED_AT_FIELD_NUMBER: _ClassVar[int]
        host: str
        name: str
        pid: int
        message: str
        created_at: Timestamp
        def __init__(self, host: _Optional[str] = ..., name: _Optional[str] = ..., pid: _Optional[int] = ..., message: _Optional[str] = ..., created_at: _Optional[_Union[Timestamp, _Mapping]] = ...) -> None: ...
    ITEMS_FIELD_NUMBER: _ClassVar[int]
    items: _containers.RepeatedCompositeFieldContainer[SystemdRequest.Item]
    def __init__(self, items: _Optional[_Iterable[_Union[SystemdRequest.Item, _Mapping]]] = ...) -> None: ...

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

class OpenSearch(_message.Message):
    __slots__ = ("cluster_name", "status", "timed_out", "number_of_nodes", "number_of_data_nodes", "discovered_master", "discovered_cluster_manager", "active_primary_shards", "active_shards", "relocating_shards", "initializing_shards", "unassigned_shards", "delayed_unassigned_shards", "number_of_pending_tasks", "number_of_in_flight_fetch", "task_max_waiting_in_queue_millis", "active_shards_percent_as_number")
    CLUSTER_NAME_FIELD_NUMBER: _ClassVar[int]
    STATUS_FIELD_NUMBER: _ClassVar[int]
    TIMED_OUT_FIELD_NUMBER: _ClassVar[int]
    NUMBER_OF_NODES_FIELD_NUMBER: _ClassVar[int]
    NUMBER_OF_DATA_NODES_FIELD_NUMBER: _ClassVar[int]
    DISCOVERED_MASTER_FIELD_NUMBER: _ClassVar[int]
    DISCOVERED_CLUSTER_MANAGER_FIELD_NUMBER: _ClassVar[int]
    ACTIVE_PRIMARY_SHARDS_FIELD_NUMBER: _ClassVar[int]
    ACTIVE_SHARDS_FIELD_NUMBER: _ClassVar[int]
    RELOCATING_SHARDS_FIELD_NUMBER: _ClassVar[int]
    INITIALIZING_SHARDS_FIELD_NUMBER: _ClassVar[int]
    UNASSIGNED_SHARDS_FIELD_NUMBER: _ClassVar[int]
    DELAYED_UNASSIGNED_SHARDS_FIELD_NUMBER: _ClassVar[int]
    NUMBER_OF_PENDING_TASKS_FIELD_NUMBER: _ClassVar[int]
    NUMBER_OF_IN_FLIGHT_FETCH_FIELD_NUMBER: _ClassVar[int]
    TASK_MAX_WAITING_IN_QUEUE_MILLIS_FIELD_NUMBER: _ClassVar[int]
    ACTIVE_SHARDS_PERCENT_AS_NUMBER_FIELD_NUMBER: _ClassVar[int]
    cluster_name: str
    status: str
    timed_out: bool
    number_of_nodes: int
    number_of_data_nodes: int
    discovered_master: bool
    discovered_cluster_manager: bool
    active_primary_shards: int
    active_shards: int
    relocating_shards: int
    initializing_shards: int
    unassigned_shards: int
    delayed_unassigned_shards: int
    number_of_pending_tasks: int
    number_of_in_flight_fetch: int
    task_max_waiting_in_queue_millis: int
    active_shards_percent_as_number: float
    def __init__(self, cluster_name: _Optional[str] = ..., status: _Optional[str] = ..., timed_out: _Optional[bool] = ..., number_of_nodes: _Optional[int] = ..., number_of_data_nodes: _Optional[int] = ..., discovered_master: _Optional[bool] = ..., discovered_cluster_manager: _Optional[bool] = ..., active_primary_shards: _Optional[int] = ..., active_shards: _Optional[int] = ..., relocating_shards: _Optional[int] = ..., initializing_shards: _Optional[int] = ..., unassigned_shards: _Optional[int] = ..., delayed_unassigned_shards: _Optional[int] = ..., number_of_pending_tasks: _Optional[int] = ..., number_of_in_flight_fetch: _Optional[int] = ..., task_max_waiting_in_queue_millis: _Optional[int] = ..., active_shards_percent_as_number: _Optional[float] = ...) -> None: ...

class ReportRequest(_message.Message):
    __slots__ = ("host", "timestamp", "items")
    class Item(_message.Message):
        __slots__ = ("curation", "http", "postgresql", "mysql", "redis", "snmp", "opensearch")
        CURATION_FIELD_NUMBER: _ClassVar[int]
        HTTP_FIELD_NUMBER: _ClassVar[int]
        POSTGRESQL_FIELD_NUMBER: _ClassVar[int]
        MYSQL_FIELD_NUMBER: _ClassVar[int]
        REDIS_FIELD_NUMBER: _ClassVar[int]
        SNMP_FIELD_NUMBER: _ClassVar[int]
        OPENSEARCH_FIELD_NUMBER: _ClassVar[int]
        curation: Duration
        http: Http
        postgresql: PostgreSql
        mysql: MySql
        redis: Redis
        snmp: Snmp
        opensearch: OpenSearch
        def __init__(self, curation: _Optional[_Union[Duration, _Mapping]] = ..., http: _Optional[_Union[Http, _Mapping]] = ..., postgresql: _Optional[_Union[PostgreSql, _Mapping]] = ..., mysql: _Optional[_Union[MySql, _Mapping]] = ..., redis: _Optional[_Union[Redis, _Mapping]] = ..., snmp: _Optional[_Union[Snmp, _Mapping]] = ..., opensearch: _Optional[_Union[OpenSearch, _Mapping]] = ...) -> None: ...
    HOST_FIELD_NUMBER: _ClassVar[int]
    TIMESTAMP_FIELD_NUMBER: _ClassVar[int]
    ITEMS_FIELD_NUMBER: _ClassVar[int]
    host: str
    timestamp: Timestamp
    items: _containers.RepeatedCompositeFieldContainer[ReportRequest.Item]
    def __init__(self, host: _Optional[str] = ..., timestamp: _Optional[_Union[Timestamp, _Mapping]] = ..., items: _Optional[_Iterable[_Union[ReportRequest.Item, _Mapping]]] = ...) -> None: ...
